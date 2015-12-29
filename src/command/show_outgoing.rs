use client::get_outgoing;
use config::{Config, get_account_id};
use cli::parse::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}", money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_outgoing_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_outgoing(&config, &account_id) {
        Ok(outgoing) => {
            represent_money(&outgoing, &hide_currency);
            0
        },
        Err(e) => {
            error!("Unable to get outgoing: {}", e);
            1
        },
    }
}
