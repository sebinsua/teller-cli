use api::TellerClient;
use api::inform::{Money, GetIncoming};
use config::Config;
use cli::arg_types::AccountType;

use chrono::UTC;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}",
             money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_incoming_command(teller: &TellerClient,
                             config: &Config,
                             account: &AccountType,
                             hide_currency: &bool)
                             -> i32 {
    info!("Calling the show incoming command");
    let account_id = config.get_account_id(&account);
    let for_month = UTC::now().date();
    teller.get_incoming(&account_id, &for_month)
          .map(|incoming| {
              represent_money(&incoming, &hide_currency);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to get incoming: {}", err);
              1
          })
}
