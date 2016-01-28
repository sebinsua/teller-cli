use api::client::{TellerClient, ApiServiceResult, Transaction};
use chrono::{Date, UTC};

#[derive(Debug)]
pub struct TransactionsWithCurrrency {
    pub transactions: Vec<Transaction>,
    pub currency: String,
}

impl TransactionsWithCurrrency {
    pub fn new<S: Into<String>>(transactions: Vec<Transaction>,
                                currency: S)
                                -> TransactionsWithCurrrency {
        TransactionsWithCurrrency {
            transactions: transactions,
            currency: currency.into(),
        }
    }
}

pub trait GetTransactionsWithCurrency {
    fn get_transactions_with_currency(&self,
                                      account_id: &str,
                                      from: &Date<UTC>,
                                      to: &Date<UTC>)
                                      -> ApiServiceResult<TransactionsWithCurrrency>;
}

impl<'a> GetTransactionsWithCurrency for TellerClient<'a> {
    fn get_transactions_with_currency(&self,
                                      account_id: &str,
                                      from: &Date<UTC>,
                                      to: &Date<UTC>)
                                      -> ApiServiceResult<TransactionsWithCurrrency> {
        let transactions = try!(self.get_transactions(&account_id, &from, &to));

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        Ok(TransactionsWithCurrrency::new(transactions, currency))
    }
}

#[cfg(test)]
mod tests {

    use api::client::{TellerClient, generate_utc_date_from_date_str};
    use super::GetTransactionsWithCurrency;

    use hyper;
    mock_connector_in_order!(GetTransactionsFollowedByGetAccount {
        include_str!("../mocks/get-transactions.http")
        include_str!("../mocks/get-account.http")
    });

    #[test]
    fn can_get_transactions_with_currency() {
        let c = hyper::client::Client::with_connector(GetTransactionsFollowedByGetAccount::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let from = generate_utc_date_from_date_str("2015-01-01");
        let to = generate_utc_date_from_date_str("2016-01-01");
        let transactions_with_currency = teller.get_transactions_with_currency("123", &from, &to).unwrap();

        assert_eq!("GBP", transactions_with_currency.currency);
        assert_eq!(10, transactions_with_currency.transactions.len());
    }

}
