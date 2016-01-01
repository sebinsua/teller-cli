use std::io::Write;
use tabwriter::TabWriter;

use config::Config;
use client::{HistoricalAmountsWithCurrency, Account};
use cli::arg_types::OutputFormat;

pub fn to_aligned_table(table_str: &str) -> String {
    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", table_str).unwrap();
    tw.flush().unwrap();

    let aligned_table_str = String::from_utf8(tw.unwrap()).unwrap();

    aligned_table_str
}

pub fn represent_list_accounts(accounts: &Vec<Account>, config: &Config) {
    let mut accounts_table = String::new();
    accounts_table.push_str("row\taccount no.\tbalance\n");
    for (idx, account) in accounts.iter().enumerate() {
        let row_number = (idx + 1) as u32;
        let account_alias = config.get_account_alias_for_id(&account.id);
        let new_account_row = format!("{} {}\t****{}\t{}\t{}\n",
                                      row_number,
                                      account_alias,
                                      account.account_number_last_4,
                                      account.balance,
                                      account.currency);
        accounts_table = accounts_table + &new_account_row;
    }

    let accounts_str = to_aligned_table(&accounts_table);

    println!("{}", accounts_str)
}

pub fn represent_list_amounts(amount_type: &str,
                              hac: &HistoricalAmountsWithCurrency,
                              output: &OutputFormat) {
    match *output {
        OutputFormat::Spark => {
            let balance_str = hac.historical_amounts
                                 .iter()
                                 .map(|b| b.1.to_owned())
                                 .collect::<Vec<String>>()
                                 .join(" ");
            println!("{}", balance_str)
        }
        OutputFormat::Standard => {
            let mut hac_table = String::new();
            let month_cols = hac.historical_amounts
                                .iter()
                                .map(|historical_amount| historical_amount.0.to_owned())
                                .collect::<Vec<String>>()
                                .join("\t");
            hac_table.push_str(&format!("\t{}\n", month_cols));
            hac_table.push_str(&format!("{} ({})", amount_type, hac.currency));
            for historical_amount in hac.historical_amounts.iter() {
                let new_amount = format!("\t{}", historical_amount.1);
                hac_table = hac_table + &new_amount;
            }

            let hac_str = to_aligned_table(&hac_table);

            println!("{}", hac_str)
        }
    }
}
