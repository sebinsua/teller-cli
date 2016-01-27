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

#[cfg(test)]
mod tests {
    use super::Money;

    #[test]
    fn can_instantiate_money() {
        let expected_amount = "10.00";
        let expected_currency = "GBP";

        let money = Money::new(expected_amount, expected_currency);

        assert_eq!(expected_amount, money.amount);
        assert_eq!(expected_currency, money.currency);
    }

    #[test]
    fn given_money_get_balance_for_display() {
        let amount = "10.00";
        let currency = "GBP";

        let money = Money::new(amount, currency);

        let money_with_currency = money.get_balance_for_display(&false);
        let money_without_currency = money.get_balance_for_display(&true);

        assert_eq!(format!("{} {}", amount, currency), money_with_currency);
        assert_eq!(amount, money_without_currency);
    }
}
