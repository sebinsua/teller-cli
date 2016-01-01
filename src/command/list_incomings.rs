use config::{Config, get_account_id};
use client::{Incomings, get_incomings};
use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

use super::representations::represent_list_amounts;

fn represent_list_incomings(hac: &Incomings, output: &OutputFormat) {
    represent_list_amounts("incoming", &hac, &output)
}

pub fn list_incomings_command(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) -> i32 {
    info!("Calling the list incomings command");
    let account_id = get_account_id(&config, &account);
    get_incomings(&config, &account_id, &interval, &timeframe).map(|incomings| {
        represent_list_incomings(&incomings, &output);
        0
    }).unwrap_or_else(|err| {
        error!("Unable to list incomings: {}", err);
        1
    })
}
