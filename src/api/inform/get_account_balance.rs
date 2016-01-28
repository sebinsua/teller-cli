use api::client::{TellerClient, ApiServiceResult, Account};
use api::inform::Money;

pub trait GetAccountBalance {
    fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money>;
}

impl<'a> GetAccountBalance for TellerClient<'a> {
    fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money> {
        let to_money = |a: Account| Money::new(a.balance, a.currency);
        self.get_account(&account_id).map(to_money)
    }
}

#[cfg(test)]
mod tests {

    use api::client::TellerClient;
    use super::GetAccountBalance;

    use hyper;
    mock_connector!(GetAccountRequest {
        "https://api.teller.io" => include_str!("../mocks/get-account.http")
    });

    #[test]
    fn can_get_account_balance() {
        let c = hyper::client::Client::with_connector(GetAccountRequest::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let money = teller.get_account_balance("123").unwrap();

        assert_eq!("1000.00 GBP", money.get_balance_for_display(&false));
    }

}
