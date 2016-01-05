use chrono::{UTC, Datelike};

use std::str::FromStr; // Use of #from_str.

use api::client::{TellerClient, ApiServiceResult, Transaction};
use api::client::parse_utc_date_from_transaction;
use api::inform::Money;

pub trait GetOutgoing {
    fn get_outgoing(&self, account_id: &str) -> ApiServiceResult<Money>;
}

impl<'a> GetOutgoing for TellerClient<'a> {
    fn get_outgoing(&self, account_id: &str) -> ApiServiceResult<Money> {
        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        let from = UTC::today().with_day(1).unwrap();
        let transactions: Vec<Transaction> = self.raw_transactions(&account_id, 250, 1)
                                                 .unwrap_or(vec![])
                                                 .into_iter()
                                                 .filter(|t| {
                                                     let transaction_date =
                                                         parse_utc_date_from_transaction(&t);
                                                     transaction_date > from
                                                 })
                                                 .collect();

        let from_float_string_to_cent_integer = |t: &Transaction| {
            (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
        };
        let from_cent_integer_to_float_string = |amount: i64| format!("{:.2}", amount as f64 / 100f64);

        let outgoing = transactions.iter()
                                   .map(from_float_string_to_cent_integer)
                                   .filter(|ci| *ci < 0)
                                   .fold(0i64, |sum, v| sum + v);

        Ok(Money::new(from_cent_integer_to_float_string(outgoing.abs()), currency))
    }
}
