pub mod error;

use self::error::TellerClientError;
use hyper::Client;
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;

use std::io::prelude::*; // Required for read_to_string use later.

// TODO: This is only temporary...
const TOKEN: &'static str = "";

pub type ApiServiceResult<T> = Result<T, TellerClientError>;

#[derive(Debug, RustcDecodable)]
pub struct AccountResponse {
    data: Account,
}

#[derive(Debug, RustcDecodable)]
pub struct AccountsResponse {
    data: Vec<Account>,
}

#[derive(Debug, RustcDecodable)]
struct Account {
    updated_at: String,
    institution: String,
    id: String,
    currency: String,
    balance: String,
    account_number_last_4: String,
}

pub fn get_accounts() -> ApiServiceResult<AccountsResponse> {
    let client = Client::new();

    let auth_header = Authorization(
        Bearer {
            token: TOKEN.to_owned()
        }
    );

    let mut res = try!(
        client.get("https://api.teller.io/accounts")
              .header(auth_header)
              .send()
    );
    if res.status.is_client_error() {
        return Err(TellerClientError::AuthenticationError);
    }

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    println!("Response: {}", body);
    let accounts_response = try!(json::decode(&body));

    Ok(accounts_response)
}

#[allow(dead_code)]
pub fn get_account() -> ApiServiceResult<AccountResponse> {
    let client = Client::new();

    let auth_header = Authorization(
        Bearer {
            token: TOKEN.to_owned()
        }
    );

    let mut res = try!(
        client.get("https://api.teller.io/accounts/4803f712-cc3e-4560-9f80-3be8116d7723")
              .header(auth_header)
              .send()
    );
    if res.status.is_client_error() {
        return Err(TellerClientError::AuthenticationError);
    }

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    println!("Response: {}", body);
    let account_response = try!(json::decode(&body));

    Ok(account_response)
}

#[allow(dead_code)]
pub fn get_account_balance() -> ApiServiceResult<String> {
    get_account().map(|r| r.data.balance)
}
