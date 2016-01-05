use cli::arg_types::Timeframe;

use super::{TellerClient, ApiServiceResult, Transaction};

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
