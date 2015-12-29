use config::Config;
use client::{Account, get_accounts};

use std::io::Write;
use tabwriter::TabWriter;

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

pub fn list_accounts_command(config: &Config) -> i32 {
    match get_accounts(&config) {
        Ok(accounts) => {
            represent_list_accounts(&accounts, &config);
            0
        },
        Err(e) => {
            error!("Unable to list accounts: {}", e);
            1
        },
    }
}
