pub mod error;

use config::Config;

use hyper::Client;
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;

use std::io::prelude::*; // Required for read_to_string use later.

use self::error::TellerClientError;

#[derive(Debug)]
pub enum Interval {
    Monthly,
    None
}

#[derive(Debug)]
pub enum Timeframe {
    Year,
    None
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

pub fn get_account(config: &Config, account_id: String) -> ApiServiceResult<Account> {
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
    get_account(&config, account_id).map(to_balance_tuple)
}

pub fn raw_transactions(config: &Config, account_id: String, count: u32, page: u32) -> ApiServiceResult<Vec<Transaction>> {
    let client = Client::new();

    let mut res = try!(
        client.get(&format!("https://api.teller.io/accounts/{}/transactions", account_id))
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

pub fn get_transactions(config: &Config, account_id: String, timeframe: &Timeframe) -> ApiServiceResult<Vec<Transaction>> {
    // TODO: Create a query string for `raw_transactions`

    // TODO:
    // install chrono: https://github.com/lifthrasiir/rust-chrono
    // switch rustc-serialize support on with:
    // http://doc.crates.io/manifest.html#the-[features]-section

    // TODO: finish listing support
    // 1. Match Timeframe on year
    // 2. Create a to (current date) and a from (current date - year)
    // 2. Get transactions and then
    // 3. Place datas into a Vec
    // 4. Get the last datas date
    // 5. If the date is predicate: past 'from' then check to see if there is a 'links.next' and if so
    // 6. get transactions with next page
    // 7. Once predicate no longer true, filter all transactions out that are not in between from and to
    // 8. finally return the Vec.

    let count = 250;
    let page = 1;
    raw_transactions(config, account_id, count, page)
}

pub fn get_balances(config: &Config, account_id: String, interval: &Interval, timeframe: &Timeframe) /* -> ApiServiceResult<Vec<Money>> */ {
    // TODO:
    // 1. call get_account and use it to set the current balance
    // 2. call get_transactions
    // 3. and reduce transactions against the current balance adding their amount and storing the
    //    value against its own month. beginning of each month
    // 4. append current_balance to the Vec
}
