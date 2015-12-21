pub mod error;

use config::Config;

use hyper::{Client, Url};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;
use chrono::{Date, DateTime, UTC};
use chrono::duration::Duration;
use itertools::Itertools;

use std::io::prelude::*; // Required for read_to_string use later.
use std::str::FromStr;

use self::error::TellerClientError;

#[derive(Debug)]
pub enum Interval {
    Monthly,
}

#[derive(Debug)]
pub enum Timeframe {
    Year,
    SixMonths,
    ThreeMonths,
}

pub type ApiServiceResult<T> = Result<T, TellerClientError>;

pub type Money = (String, String);

#[derive(Debug, RustcDecodable)]
struct AccountResponse {
    data: Account,
}

#[derive(Debug, RustcDecodable)]
struct AccountsResponse {
    data: Vec<Account>,
}

#[derive(Debug, RustcDecodable)]
struct TransactionsResponse {
    data: Vec<Transaction>,
}

#[derive(Debug, RustcDecodable)]
pub struct Account {
    pub updated_at: String,
    pub institution: String,
    pub id: String,
    pub currency: String,
    pub balance: String,
    pub account_number_last_4: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Transaction {
    pub description: String,
    pub date: String,
    pub counterparty: String,
    pub amount: String,
}

#[derive(Debug)]
pub struct Balances {
    pub historical_amounts: Vec<(String, String)>,
    pub currency: String,
}

impl Balances {
    pub fn new(historical_amounts: Vec<(String, String)>, currency: String) -> Balances {
        Balances {
            historical_amounts: historical_amounts,
            currency: currency,
        }
    }
}

fn get_auth_header(auth_token: &str) -> Authorization<Bearer> {
    Authorization(
        Bearer {
            token: auth_token.to_string(),
        }
    )
}

pub fn get_accounts(config: &Config) -> ApiServiceResult<Vec<Account>> {
    let client = Client::new();

    let mut res = try!(
        client.get("https://api.teller.io/accounts")
              .header(get_auth_header(&config.auth_token))
              .send()
    );
    if res.status.is_client_error() {
        return Err(TellerClientError::AuthenticationError);
    }

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    debug!("GET /accounts response: {}", body);

    let accounts_response: AccountsResponse = try!(json::decode(&body));

    Ok(accounts_response.data)
}

pub fn get_account(config: &Config, account_id: &str) -> ApiServiceResult<Account> {
    let client = Client::new();

    let mut res = try!(
        client.get(&format!("https://api.teller.io/accounts/{}", account_id))
              .header(get_auth_header(&config.auth_token))
              .send()
    );
    if res.status.is_client_error() {
        return Err(TellerClientError::AuthenticationError);
    }

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    debug!("GET /account/:id response: {}", body);

    let account_response: AccountResponse = try!(json::decode(&body));

    Ok(account_response.data)
}

pub fn get_account_balance(config: &Config, account_id: &str) -> ApiServiceResult<Money> {
    let to_balance_tuple = |a: Account| (a.balance, a.currency);
    get_account(&config, &account_id).map(to_balance_tuple)
}

pub fn raw_transactions(config: &Config, account_id: &str, count: u32, page: u32) -> ApiServiceResult<Vec<Transaction>> {
    let mut url = Url::parse(&format!("https://api.teller.io/accounts/{}/transactions", account_id)).unwrap();

    const COUNT: &'static str = "count";
    const PAGE: &'static str = "page";
    let query = vec![ (COUNT, count.to_string()), (PAGE, page.to_string()) ];
    url.set_query_from_pairs(query.into_iter());

    let client = Client::new();
    let mut res = try!(
        client.get(url)
              .header(get_auth_header(&config.auth_token))
              .send()
    );
    if res.status.is_client_error() {
        return Err(TellerClientError::AuthenticationError);
    }

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    debug!("GET /account/:id/transactions response: {}", body);

    let transactions_response: TransactionsResponse = try!(json::decode(&body));

    Ok(transactions_response.data)
}

fn parse_utc_date_from_transaction(t: &Transaction) -> Date<UTC> {
    let full_date = &(t.date.to_owned() + "T00:00:00-00:00");
    let past_transaction_date_without_tz = DateTime::parse_from_rfc3339(full_date).unwrap().date();
    let past_transaction_date = past_transaction_date_without_tz.with_timezone(&UTC);
    past_transaction_date
}

pub fn get_transactions(config: &Config, account_id: &str, timeframe: &Timeframe) -> ApiServiceResult<Vec<Transaction>> {
    let page_through_transactions = |from| -> ApiServiceResult<Vec<Transaction>> {
        let mut all_transactions = vec![];

        let mut fetching = true;
        let mut page = 1;
        let count = 250;
        while fetching {
            let mut transactions = try!(raw_transactions(config, &account_id, count, page));
            match transactions.last() {
                None => {
                    // If there are no transactions left, do not fetch forever...
                    fetching = false
                },
                Some(past_transaction) => {
                    let past_transaction_date = parse_utc_date_from_transaction(&past_transaction);
                    if past_transaction_date < from {
                        fetching = false;
                    }
                },
            };

            all_transactions.append(&mut transactions);
            page = page + 1;
        }

        all_transactions = all_transactions.into_iter().filter(|t| {
            let transaction_date = parse_utc_date_from_transaction(&t);
            transaction_date > from
        }).collect();

        all_transactions.reverse();
        Ok(all_transactions)
    };

    match *timeframe {
        Timeframe::ThreeMonths => {
            let to = UTC::today();
            let from = to - Duration::days(91); // close enough... 😅

            page_through_transactions(from)
        },
        Timeframe::SixMonths => {
            let to = UTC::today();
            let from = to - Duration::days(183);

            page_through_transactions(from)
        },
        Timeframe::Year => {
            let to = UTC::today();
            let from = to - Duration::days(365);

            page_through_transactions(from)
        },
    }
}

pub fn get_balances(config: &Config, account_id: &str, interval: &Interval, timeframe: &Timeframe) -> ApiServiceResult<Balances> {
    let transactions: Vec<Transaction> = get_transactions(&config, &account_id, &timeframe).unwrap_or(vec![]);

    let mut month_year_total_transactions: Vec<(String, i64)> = transactions.into_iter().group_by(|t| {
        let transaction_date = parse_utc_date_from_transaction(&t);
        match *interval {
            Interval::Monthly => {
                let group_name = transaction_date.format("%m-%Y").to_string();
                group_name
            }
        }
    }).map(|myt| {
        let group_name = myt.0;
        let amount = myt.1.into_iter().map(|t| {
            let v = (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64;
            v
        }).fold(0i64, |sum, v| sum + v);
        (group_name, amount)
    }).collect();
    month_year_total_transactions.reverse();

    let account = try!(get_account(&config, &account_id));
    let current_balance = (f64::from_str(&account.balance).unwrap() * 100f64).round() as i64;
    let currency = account.currency;

    let mut historical_amounts: Vec<(String, String)> = vec![];
    historical_amounts.push(("current".to_string(), format!("{:.2}", current_balance as f64 / 100f64)));

    let mut last_balance = current_balance;
    for mytt in month_year_total_transactions {
        last_balance = last_balance - mytt.1;
        historical_amounts.push((mytt.0.to_string(), format!("{:.2}", last_balance as f64 / 100f64)));
    }
    historical_amounts.reverse();

    Ok(Balances::new(historical_amounts, currency))
}
