use config::{Config, get_account_id};
use client::get_counterparties;
use cli::arg_types::{AccountType, Timeframe};

use std::io::Write;
use tabwriter::TabWriter;

fn represent_list_counterparties(counterparties: &Vec<(String, String)>, currency: &str, count: &i64) {
    let mut counterparties_table = String::new();

    counterparties_table.push_str(&format!("row\tcounterparty\tamount ({})\n", currency));
    let skip_n = counterparties.len() - (*count as usize);
    for (idx, counterparty) in counterparties.iter().skip(skip_n).enumerate() {
        let row_number = (idx + 1) as u32;
        let new_counterparty_row = format!("{}\t{}\t{}\n", row_number, counterparty.0, counterparty.1);
        counterparties_table = counterparties_table + &new_counterparty_row;
    }

    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", counterparties_table).unwrap();
    tw.flush().unwrap();

    let counterparties_str = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", counterparties_str)
}

pub fn list_counterparties_command(config: &Config, account: &AccountType, timeframe: &Timeframe, count: &i64) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_counterparties(&config, &account_id, &timeframe) {
        Ok(counterparties_with_currency) => {
            represent_list_counterparties(&counterparties_with_currency.counterparties, &counterparties_with_currency.currency, &count);
            0
        },
        Err(e) => {
            error!("Unable to list counterparties: {}", e);
            1
        },
    }
}
