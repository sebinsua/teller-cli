use client::get_incoming;
use config::{Config, get_account_id};
use cli::arg_types::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}",
             money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_incoming_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    info!("Calling the show incoming command");
    let account_id = get_account_id(&config, &account);
    get_incoming(&config, &account_id)
        .map(|incoming| {
            represent_money(&incoming, &hide_currency);
            0
        })
        .unwrap_or_else(|err| {
            error!("Unable to get incoming: {}", err);
            1
        })
}
