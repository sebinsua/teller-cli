use std::io::Write;
use tabwriter::TabWriter;

use config::Config;
use client::{HistoricalAmountsWithCurrency, Account};
use cli::arg_types::OutputFormat;

fn get_account_alias_for_id<'a>(account_id: &str, config: &Config) -> &'a str {
    if *account_id == config.current {
        "(current)"
    } else if *account_id == config.savings {
        "(savings)"
    } else if *account_id == config.business {
        "(business)"
    } else {
        ""
    }
}

pub fn represent_list_accounts(accounts: &Vec<Account>, config: &Config) {
    let mut accounts_table = String::new();
    accounts_table.push_str("row\taccount no.\tbalance\n");
    for (idx, account) in accounts.iter().enumerate() {
        let row_number = (idx + 1) as u32;
        let account_alias = get_account_alias_for_id(&account.id, &config);
        let new_account_row = format!("{} {}\t****{}\t{}\t{}\n", row_number, account_alias, account.account_number_last_4, account.balance, account.currency);
        accounts_table = accounts_table + &new_account_row;
    }

    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", accounts_table).unwrap();
    tw.flush().unwrap();

    let accounts_str = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", accounts_str)
}

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
