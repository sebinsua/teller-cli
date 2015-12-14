pub mod error;

use config::Config;

use hyper::Client;
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;

use std::io::prelude::*; // Required for read_to_string use later.

use self::error::TellerClientError;

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
pub struct Account {
    pub updated_at: String,
    pub institution: String,
    pub id: String,
    pub currency: String,
    pub balance: String,
    pub account_number_last_4: String,
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

pub fn get_account_balance(config: &Config, account_id: String) -> ApiServiceResult<(String, String)> {
    let to_balance_tuple = |a: Account| (a.balance, a.currency);
    get_account(&config, account_id).map(to_balance_tuple)
}
