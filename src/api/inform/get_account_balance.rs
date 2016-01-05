use super::{TellerClient, ApiServiceResult, Account, Money};

pub trait GetAccountBalance {
    fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money>;
}

impl<'a> GetAccountBalance for TellerClient<'a> {
    fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money> {
        let to_money = |a: Account| Money::new(a.balance, a.currency);
        self.get_account(&account_id).map(to_money)
    }
}
