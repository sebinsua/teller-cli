use config::{Config, get_account_id};
use client::{HistoricalAmountsWithCurrency, Balances, Interval, Timeframe, get_balances};
use cli::parse::{AccountType, OutputFormat};

use std::io::Write;
use tabwriter::TabWriter;

pub fn represent_list_amounts(amount_type: &str, hac: &HistoricalAmountsWithCurrency, output: &OutputFormat) {
    match *output {
        OutputFormat::Spark => {
            let balance_str = hac.historical_amounts.iter().map(|b| b.1.to_owned()).collect::<Vec<String>>().join(" ");
            println!("{}", balance_str)
        },
        OutputFormat::Standard => {
            let mut hac_table = String::new();
            let month_cols = hac.historical_amounts.iter().map(|historical_amount| historical_amount.0.to_owned()).collect::<Vec<String>>().join("\t");
            hac_table.push_str(&format!("\t{}\n", month_cols));
            hac_table.push_str(&format!("{} ({})", amount_type, hac.currency));
            for historical_amount in hac.historical_amounts.iter() {
                let new_amount = format!("\t{}", historical_amount.1);
                hac_table = hac_table + &new_amount;
            }

            let mut tw = TabWriter::new(Vec::new());
            write!(&mut tw, "{}", hac_table).unwrap();
            tw.flush().unwrap();

            let hac_str = String::from_utf8(tw.unwrap()).unwrap();

            println!("{}", hac_str)
        },
    }
}

fn represent_list_balances(hac: &Balances, output: &OutputFormat) {
    represent_list_amounts("balance", &hac, &output)
}

pub fn list_balances_command(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_balances(&config, &account_id, &interval, &timeframe) {
        Ok(balances) => {
            represent_list_balances(&balances, &output);
            0
        },
        Err(e) => {
            error!("Unable to list balances: {}", e);
            1
        },
    }
}
