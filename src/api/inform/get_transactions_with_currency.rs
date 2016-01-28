use cli::arg_types::Timeframe;

use api::client::{TellerClient, ApiServiceResult, Transaction};

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
                                      timeframe: &Timeframe)
                                      -> ApiServiceResult<TransactionsWithCurrrency>;
}

impl<'a> GetTransactionsWithCurrency for TellerClient<'a> {
    fn get_transactions_with_currency(&self,
                                      account_id: &str,
                                      timeframe: &Timeframe)
                                      -> ApiServiceResult<TransactionsWithCurrrency> {
        let transactions = try!(self.get_transactions(&account_id, &timeframe));

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        Ok(TransactionsWithCurrrency::new(transactions, currency))
    }
}

#[cfg(test)]
mod tests {

    use api::client::TellerClient;
    use super::GetTransactionsWithCurrency;

    use cli::arg_types::Timeframe;

    use hyper;
    mock_connector_in_order!(GetTransactionsFollowedByGetAccount {
        include_str!("../mocks/get-transactions.http")
        include_str!("../mocks/get-account.http")
    });

    #[test]
    fn can_get_transactions_with_currency() {
        let c = hyper::client::Client::with_connector(GetTransactionsFollowedByGetAccount::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let transactions_with_currency = teller.get_transactions_with_currency("123", &Timeframe::ThreeMonths).unwrap();

        assert_eq!("GBP", transactions_with_currency.currency);
        // NOTE: I can't test transactions because get_transactions filters based on the time
        // which I currently cannot control.
    }

}
