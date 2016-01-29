use hyper::{Client, Url};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;
use chrono::{Date, DateTime, UTC};

use std::io::prelude::*; // Required for read_to_string use later.

use api::error::TellerClientError;

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

#[derive(Debug, RustcDecodable, Clone)]
pub struct Account {
    pub name: String,
    pub institution: String,
    pub id: String,
    pub currency: String,
    pub bank_code: String,
    pub balance: String,
    pub account_number_last_4: String,
}

#[derive(Debug, RustcDecodable, Clone)]
pub struct Transaction {
    pub description: String,
    pub date: String,
    pub counterparty: String,
    pub amount: String,
}

pub fn parse_utc_date_from_transaction(t: &Transaction) -> Date<UTC> {
    generate_utc_date_from_date_str(&t.date)
}

pub fn generate_utc_date_from_date_str(d: &str) -> Date<UTC> {
    let full_date = &(d.to_owned() + "T00:00:00-00:00");
    let date_without_tz = DateTime::parse_from_rfc3339(full_date).unwrap().date();
    let date = date_without_tz.with_timezone(&UTC);
    date
}

const TELLER_API_SERVER_URL: &'static str = "https://api.teller.io";

pub struct TellerClient<'a> {
    client: Client,
    auth_token: &'a str,
}

impl<'a> TellerClient<'a> {
    pub fn new(auth_token: &'a str) -> TellerClient {
        let client = Client::new();
        TellerClient {
            client: client,
            auth_token: auth_token,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_hyper_client(auth_token: &'a str, client: Client) -> TellerClient {
        TellerClient {
            client: client,
            auth_token: auth_token,
        }
    }

    fn get_body(&self, url: &str) -> ApiServiceResult<String> {
        let mut res = try!(self.client.get(url)
                               .header(Authorization(
                                   Bearer { token: self.auth_token.to_string() }
                               )).send());
        if res.status.is_client_error() {
            return Err(TellerClientError::AuthenticationError);
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        info!("GET {}", url);
        debug!("Response: {}", body);

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
                            page_size: u32,
                            page: u32)
                            -> ApiServiceResult<Vec<Transaction>> {
        let mut url = Url::parse(&format!("{}/accounts/{}/transactions",
                                          TELLER_API_SERVER_URL,
                                          account_id)).unwrap();

        const PAGE_SIZE: &'static str = "page_size";
        const PAGE: &'static str = "page";
        let query = vec![(PAGE_SIZE, page_size.to_string()), (PAGE, page.to_string())];
        url.set_query_from_pairs(query.into_iter());

        let body = try!(self.get_body(&url.serialize()));
        let transactions_response: TransactionsResponse = try!(json::decode(&body));

        Ok(transactions_response.data)
    }

    #[allow(unused_variables)]
    pub fn get_transactions(&self,
                            account_id: &str,
                            from: &Date<UTC>,
                            to: &Date<UTC>)
                            -> ApiServiceResult<Vec<Transaction>> {
        let page_through_transactions = |from| -> ApiServiceResult<Vec<Transaction>> {
            let mut all_transactions = vec![];

            let mut fetching = true;
            let mut page = 1;
            let page_size = 250;
            while fetching {
                let mut transactions = try!(self.raw_transactions(&account_id, page_size, page));
                match transactions.last() {
                    None => {
                        // If there are no transactions left, do not fetch forever...
                        fetching = false
                    }
                    Some(past_transaction) => {
                        let past_transaction_date = parse_utc_date_from_transaction(&past_transaction);
                        if past_transaction_date <= from {
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
                                                   transaction_date >= from
                                               })
                                               .collect();

            all_transactions.reverse();
            Ok(all_transactions)
        };

        page_through_transactions(*from)
    }

}

#[cfg(test)]
mod tests {
    use super::{TellerClient, Account, Transaction, generate_utc_date_from_date_str};

    use std::error::Error;
    use hyper;

    mock_connector!(FailAuthenticationRequest {
        "https://api.teller.io" => include_str!("./mocks/fail-authentication.http")
    });
    mock_connector!(GetAccountRequest {
        "https://api.teller.io" => include_str!("./mocks/get-account.http")
    });
    mock_connector!(GetAccountsRequest {
        "https://api.teller.io" => include_str!("./mocks/get-accounts.http")
    });
    mock_connector!(GetTransactionsRequest {
        "https://api.teller.io" => include_str!("./mocks/get-transactions.http")
    });

    #[test]
    #[allow(unused)]
    fn can_instantiate_teller_client() {
        let client = TellerClient::new("auth_token");
        assert!(true);
    }

    #[test]
    #[allow(unused)]
    fn can_instantiate_account() {
        let account = Account {
            name: "Current Account".to_string(),
            institution: "Natwest".to_string(),
            id: "some-long-uuid".to_string(),
            currency: "GBP".to_string(),
            bank_code: "00000000".to_string(),
            balance: "1000.00".to_string(),
            account_number_last_4: "0000".to_string(),
        };
        assert!(true);
    }

    #[test]
    #[allow(unused)]
    fn can_instantiate_transaction() {
        let transaction = Transaction {
            description: "4836 19JAN16 C , NANNA'S , LONDON GB".to_string(),
            date: "2016-01-21".to_string(),
            counterparty: "NANNA'S".to_string(),
            amount: "-10.00".to_string(),
        };
        assert!(true);
    }

    #[test]
    fn can_fail_authentication() {
        let c = hyper::client::Client::with_connector(FailAuthenticationRequest::default());
        let client = TellerClient::new_with_hyper_client("fake-auth-token", c);
        let get_account_state = client.get_account("123");

        assert_eq!(true, get_account_state.is_err());

        let auth_err = get_account_state.unwrap_err();
        assert_eq!("Could not authenticate", auth_err.description());
    }

    #[test]
    fn can_get_account() {
        let c = hyper::client::Client::with_connector(GetAccountRequest::default());
        let client = TellerClient::new_with_hyper_client("fake-auth-token", c);
        let account = client.get_account("123").unwrap();

        assert_eq!("123", account.id);
        assert_eq!("natwest", account.institution);
        assert_eq!("current", account.name);
        assert_eq!("1000.00", account.balance);
        assert_eq!("GBP", account.currency);
        assert_eq!("000000", account.bank_code);
        assert_eq!("0000", account.account_number_last_4);
    }

    #[test]
    fn can_get_accounts() {
        let c = hyper::client::Client::with_connector(GetAccountsRequest::default());
        let client = TellerClient::new_with_hyper_client("fake-auth-token", c);
        let accounts = client.get_accounts().unwrap();

        assert_eq!("Savings", accounts[0].name);
        assert_eq!("Current", accounts[1].name);
    }

    #[test]
    fn can_get_transactions() {
        let c = hyper::client::Client::with_connector(GetTransactionsRequest::default());
        let client = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let from = generate_utc_date_from_date_str("2015-01-01");
        let to = generate_utc_date_from_date_str("2016-01-01");
        let transactions = client.get_transactions("123", &from, &to).unwrap();

        assert_eq!("COUNTERPARTY-1", transactions[9].counterparty);
        assert_eq!("COUNTERPARTY-2", transactions[8].counterparty);
    }

}
