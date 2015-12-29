use client::get_account_balance;
use config::{Config, get_account_id};
use cli::parse::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}", money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_balance_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_account_balance(&config, &account_id) {
        Ok(balance) => {
            represent_money(&balance, &hide_currency);
            0
        },
        Err(e) => {
            error!("Unable to get account balance: {}", e);
            1
        },
    }
}
