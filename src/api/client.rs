use cli::arg_types::Timeframe;

use hyper::{Client, Url};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;
use chrono::{Date, DateTime, UTC};
use chrono::duration::Duration;

use std::io::prelude::*; // Required for read_to_string use later.

use super::error::TellerClientError;

pub type ApiServiceResult<T> = Result<T, TellerClientError>;

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

pub fn parse_utc_date_from_transaction(t: &Transaction) -> Date<UTC> {
    let full_date = &(t.date.to_owned() + "T00:00:00-00:00");
    let past_transaction_date_without_tz = DateTime::parse_from_rfc3339(full_date).unwrap().date();
    let past_transaction_date = past_transaction_date_without_tz.with_timezone(&UTC);
    past_transaction_date
}

const TELLER_API_SERVER_URL: &'static str = "https://api.teller.io";

pub struct TellerClient<'a> {
    auth_token: &'a str,
}

impl<'a> TellerClient<'a> {
    pub fn new(auth_token: &'a str) -> TellerClient {
        TellerClient {
            auth_token: auth_token,
        }
    }

    fn get_body(&self, url: &str) -> ApiServiceResult<String> {
        let client = Client::new();
        let mut res = try!(client.get(url)
                                 .header(Authorization(
                                     Bearer { token: self.auth_token.to_string() }
                                 ))
                                 .send());
        if res.status.is_client_error() {
            return Err(TellerClientError::AuthenticationError);
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        debug!("GET {} response: {}", url, body);

        Ok(body)
    }

    pub fn get_accounts(&self) -> ApiServiceResult<Vec<Account>> {
        let body = try!(self.get_body(&format!("{}/accounts", TELLER_API_SERVER_URL)));
        let accounts_response: AccountsResponse = try!(json::decode(&body));

        Ok(accounts_response.data)
    }

    pub fn get_account(&self, account_id: &str) -> ApiServiceResult<Account> {
        let body = try!(self.get_body(&format!("{}/accounts/{}", TELLER_API_SERVER_URL, account_id)));
        let account_response: AccountResponse = try!(json::decode(&body));

        Ok(account_response.data)
    }

    pub fn raw_transactions(&self,
                            account_id: &str,
                            count: u32,
                            page: u32)
                            -> ApiServiceResult<Vec<Transaction>> {
        let mut url = Url::parse(&format!("{}/accounts/{}/transactions",
                                          TELLER_API_SERVER_URL,
                                          account_id)).unwrap();

        const COUNT: &'static str = "count";
        const PAGE: &'static str = "page";
        let query = vec![(COUNT, count.to_string()), (PAGE, page.to_string())];
        url.set_query_from_pairs(query.into_iter());

        let body = try!(self.get_body(&url.serialize()));
        let transactions_response: TransactionsResponse = try!(json::decode(&body));

        Ok(transactions_response.data)
    }

    pub fn get_transactions(&self,
                            account_id: &str,
                            timeframe: &Timeframe)
                            -> ApiServiceResult<Vec<Transaction>> {
        let page_through_transactions = |from| -> ApiServiceResult<Vec<Transaction>> {
            let mut all_transactions = vec![];

            let mut fetching = true;
            let mut page = 1;
            let count = 250;
            while fetching {
                let mut transactions = try!(self.raw_transactions(&account_id, count, page));
                match transactions.last() {
                    None => {
                        // If there are no transactions left, do not fetch forever...
                        fetching = false
                    }
                    Some(past_transaction) => {
                        let past_transaction_date = parse_utc_date_from_transaction(&past_transaction);
                        if past_transaction_date < from {
                            fetching = false;
                        }
                    }
                };

                all_transactions.append(&mut transactions);
                page = page + 1;
            }

            all_transactions = all_transactions.into_iter()
                                               .filter(|t| {
                                                   let transaction_date =
                                                       parse_utc_date_from_transaction(&t);
                                                   transaction_date > from
                                               })
                                               .collect();

            all_transactions.reverse();
            Ok(all_transactions)
        };

        match *timeframe {
            Timeframe::ThreeMonths => {
                let to = UTC::today();
                let from = to - Duration::days(91); // close enough... 😅

                page_through_transactions(from)
            }
            Timeframe::SixMonths => {
                let to = UTC::today();
                let from = to - Duration::days(183);

                page_through_transactions(from)
            }
            Timeframe::Year => {
                let to = UTC::today();
                let from = to - Duration::days(365);

                page_through_transactions(from)
            }
        }
    }

}
