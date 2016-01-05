pub mod get_account_balance;
pub mod get_incoming;
pub mod get_outgoing;
pub mod get_transactions_with_currency;
pub mod get_counterparties;
pub mod get_aggregates;

pub use api::client::{TellerClient, ApiServiceResult, Transaction, Account};

pub use self::get_account_balance::*;
pub use self::get_incoming::*;
pub use self::get_outgoing::*;
pub use self::get_transactions_with_currency::*;
pub use self::get_counterparties::*;
pub use self::get_aggregates::*;

#[derive(Debug)]
pub struct Money {
    amount: String,
    currency: String,
}

impl Money {
    pub fn new<S: Into<String>>(amount: S, currency: S) -> Money {
        Money {
            amount: amount.into(),
            currency: currency.into(),
        }
    }

    pub fn get_balance_for_display(&self, hide_currency: &bool) -> String {
        if *hide_currency {
            self.amount.to_owned()
        } else {
            let balance_with_currency = format!("{} {}", self.amount, self.currency);
            balance_with_currency.to_owned()
        }
    }
}
