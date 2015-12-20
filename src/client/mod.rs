pub mod error;

use config::Config;

use hyper::{Client, Url};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;
use chrono::{Date, DateTime, UTC, Datelike};
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

fn get_auth_header(auth_token: &String) -> Authorization<Bearer> {
    Authorization(
        Bearer {
            token: auth_token.to_owned()
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

pub fn get_account_balance(config: &Config, account_id: String) -> ApiServiceResult<Money> {
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

// TODO: Fix number precision in string
// TODO: `.format` date appropriately using http://lifthrasiir.github.io/rust-chrono/chrono/format/strftime/index.html
// TODO: Create `Balances` struct with a better shape (currency, historical_amounts (String, String)).
// TODO: Use the TabWriter to write this to stdout effectively.
pub fn get_balances(config: &Config, account_id: &str, interval: &Interval, timeframe: &Timeframe) -> ApiServiceResult<Vec<Money>> {
    let mut balances: Vec<Money> = vec![];
    let transactions: Vec<Transaction> = get_transactions(&config, &account_id, &timeframe).unwrap_or(vec![]);

    let month_year_total_transactions: Vec<(String, f32)> = transactions.into_iter().group_by(|t| {
        let transaction_date = parse_utc_date_from_transaction(&t);
        match *interval {
            Interval::Monthly => {
                let group_name = format!("{}-{}", transaction_date.month(), transaction_date.year());
                group_name
            }
        }
    }).map(|myt| {
        let amount = myt.1.into_iter().map(|t| {
            f32::from_str(&t.amount).unwrap()
        }).fold(0f32, |sum, v| sum + v);
        (myt.0, amount)
    }).collect();

    let current_balance = try!(match get_account(&config, &account_id) {
        Ok(ref account) => {
            Ok((account.balance.to_owned(), account.currency.to_owned()))
        },
        Err(e) => Err(e),
    });
    balances.push(current_balance.to_owned());
    let mut last_balance = f32::from_str(&current_balance.0).unwrap();
    for mytt in month_year_total_transactions {
        last_balance = last_balance - mytt.1;
        balances.push((last_balance.to_string(), current_balance.1.to_owned()));
    }

    balances.reverse();
    Ok(balances)
}
