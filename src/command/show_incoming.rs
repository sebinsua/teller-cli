use client::TellerClient;
use config::Config;
use cli::arg_types::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}",
             money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_incoming_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    info!("Calling the show incoming command");
    let account_id = config.get_account_id(&account);
    let teller = TellerClient::new(&config.auth_token);
    teller.get_incoming(&account_id)
          .map(|incoming| {
              represent_money(&incoming, &hide_currency);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to get incoming: {}", err);
              1
          })
}
