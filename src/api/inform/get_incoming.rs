use chrono::{UTC, Date, Datelike};

use std::str::FromStr; // Use of #from_str.

use api::client::{TellerClient, ApiServiceResult, Transaction};
use api::client::parse_utc_date_from_transaction;
use api::inform::Money;

pub trait GetIncoming {
    fn get_incoming(&self, account_id: &str, for_month: &Date<UTC>) -> ApiServiceResult<Money>;
}

impl<'a> GetIncoming for TellerClient<'a> {
    fn get_incoming(&self, account_id: &str, for_month: &Date<UTC>) -> ApiServiceResult<Money> {
        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        let from = for_month.with_day(1).unwrap();
        let to = from.with_month(from.month() + 1).unwrap();
        let transactions: Vec<Transaction> = self.raw_transactions(&account_id, 250, 1)
                                                 .unwrap_or(vec![])
                                                 .into_iter()
                                                 .filter(|t| {
                                                     let transaction_date =
                                                         parse_utc_date_from_transaction(&t);
                                                     from <= transaction_date && transaction_date <= to
                                                 })
                                                 .collect();

        let from_float_string_to_cent_integer = |t: &Transaction| {
            (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
        };
        let from_cent_integer_to_float_string = |amount: i64| format!("{:.2}", amount as f64 / 100f64);

        let incoming = transactions.iter()
                                   .map(from_float_string_to_cent_integer)
                                   .filter(|ci| *ci > 0)
                                   .fold(0i64, |sum, v| sum + v);

        Ok(Money::new(from_cent_integer_to_float_string(incoming), currency))
    }
}

#[cfg(test)]
mod tests {

    use api::client::{TellerClient, generate_utc_date_from_date_str};
    use super::GetIncoming;

    use hyper;
    mock_connector_in_order!(GetAccountFollowedByGetTransactions {
        include_str!("../mocks/get-account.http")
        include_str!("../mocks/get-transactions.http")
    });

    #[test]
    fn can_get_incoming() {
        let c = hyper::client::Client::with_connector(GetAccountFollowedByGetTransactions::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let current_month = generate_utc_date_from_date_str("2016-01-01");
        let money = teller.get_incoming("123", &current_month).unwrap();

        assert_eq!("0.00 GBP", money.get_balance_for_display(&false));
    }

}
