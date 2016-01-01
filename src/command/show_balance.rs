use client::get_account_balance;
use config::Config;
use cli::arg_types::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}",
             money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_balance_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    info!("Calling the show balance command");
    let account_id = config.get_account_id(&account);
    get_account_balance(&config, &account_id)
        .map(|balance| {
            represent_money(&balance, &hide_currency);
            0
        })
        .unwrap_or_else(|err| {
            error!("Unable to get account balance: {}", err);
            1
        })
}
