use config::{Config, get_account_id};
use client::{Transaction, Timeframe, get_transactions_with_currency};
use cli::parse::AccountType;

use std::io::Write;
use tabwriter::TabWriter;

fn represent_list_transactions(transactions: &Vec<Transaction>, currency: &str, show_description: &bool) {
    let mut transactions_table = String::new();

    if *show_description {
        transactions_table.push_str(&format!("row\tdate\tcounterparty\tamount ({})\tdescription\n", currency));
        for (idx, transaction) in transactions.iter().enumerate() {
            let row_number = (idx + 1) as u32;
            let new_transaction_row = format!("{}\t{}\t{}\t{}\t{}\n", row_number, transaction.date, transaction.counterparty, transaction.amount, transaction.description);
            transactions_table = transactions_table + &new_transaction_row;
        }
    } else {
        transactions_table.push_str(&format!("row\tdate\tcounterparty\tamount ({})\n", currency));
        for (idx, transaction) in transactions.iter().enumerate() {
            let row_number = (idx + 1) as u32;
            let new_transaction_row = format!("{}\t{}\t{}\t{}\n", row_number, transaction.date, transaction.counterparty, transaction.amount);
            transactions_table = transactions_table + &new_transaction_row;
        }
    }

    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", transactions_table).unwrap();
    tw.flush().unwrap();

    let transactions_str = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", transactions_str)
}

pub fn list_transactions_command(config: &Config, account: &AccountType, timeframe: &Timeframe, show_description: &bool) -> i32 {
    let account_id = get_account_id(&config, &account);
    match get_transactions_with_currency(&config, &account_id, &timeframe) {
        Ok(transactions_with_currency) => {
            represent_list_transactions(&transactions_with_currency.transactions, &transactions_with_currency.currency, &show_description);
            0
        },
        Err(e) => {
            error!("Unable to list transactions: {}", e);
            1
        },
    }
}
