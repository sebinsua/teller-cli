use client::get_outgoing;
use config::Config;
use cli::arg_types::AccountType;
use client::Money;

fn represent_money(money_with_currency: &Money, hide_currency: &bool) {
    println!("{}",
             money_with_currency.get_balance_for_display(&hide_currency))
}

pub fn show_outgoing_command(config: &Config, account: &AccountType, hide_currency: &bool) -> i32 {
    info!("Calling the show outgoing command");
    let account_id = config.get_account_id(&account);
    get_outgoing(&config, &account_id)
        .map(|outgoing| {
            represent_money(&outgoing, &hide_currency);
            0
        })
        .unwrap_or_else(|err| {
            error!("Unable to get outgoing: {}", err);
            1
        })
}
