use config::{Config, get_account_id};
use client::{Balances, get_balances};
use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

use super::representations::represent_list_amounts;

fn represent_list_balances(hac: &Balances, output: &OutputFormat) {
    represent_list_amounts("balance", &hac, &output)
}

pub fn list_balances_command(config: &Config,
                             account: &AccountType,
                             interval: &Interval,
                             timeframe: &Timeframe,
                             output: &OutputFormat)
                             -> i32 {
    info!("Calling the list balances command");
    let account_id = get_account_id(&config, &account);
    get_balances(&config, &account_id, &interval, &timeframe)
        .map(|balances| {
            represent_list_balances(&balances, &output);
            0
        })
        .unwrap_or_else(|err| {
            error!("Unable to list balances: {}", err);
            1
        })
}
