use config::{Config, get_account_id};
use client::{Incomings, Interval, Timeframe, get_incomings};
use cli::parse::{AccountType, OutputFormat};

use super::list_balances::represent_list_amounts;

fn represent_list_incomings(hac: &Incomings, output: &OutputFormat) {
    represent_list_amounts("incoming", &hac, &output)
}

pub fn list_incomings_command(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_incomings(&config, &account_id, &interval, &timeframe) {
        Ok(incomings) => {
            represent_list_incomings(&incomings, &output);
            0
        },
        Err(e) => {
            error!("Unable to list incomings: {}", e);
            1
        },
    }
}
