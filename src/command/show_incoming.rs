use client::get_incoming;
use config::{Config, get_account_id};
use cli::arg_types::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}", money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_incoming_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_incoming(&config, &account_id) {
        Ok(incoming) => {
            represent_money(&incoming, &hide_currency);
            0
        },
        Err(e) => {
            error!("Unable to get incoming: {}", e);
            1
        },
    }
}
