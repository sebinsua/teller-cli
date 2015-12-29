use config::{Config, get_account_id};
use client::{Outgoings, Interval, Timeframe, get_outgoings};
use cli::parse::{AccountType, OutputFormat};

use super::list_balances::represent_list_amounts;

fn represent_list_outgoings(hac: &Outgoings, output: &OutputFormat) {
    represent_list_amounts("outgoing", &hac, &output)
}

pub fn list_outgoings_command(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_outgoings(&config, &account_id, &interval, &timeframe) {
        Ok(outgoings) => {
            represent_list_outgoings(&outgoings, &output);
            0
        },
        Err(e) => {
            error!("Unable to list ougoings: {}", e);
            1
        },
    }
}
