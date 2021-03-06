use config::Config;
use api::{Transaction, TellerClient};
use api::inform::{TransactionsWithCurrrency, GetTransactionsWithCurrency};
use cli::arg_types::{Timeframe, AccountType};

use command::representations::to_aligned_table;

use command::timeframe_to_date_range;

fn represent_list_transactions(transactions: &Vec<Transaction>,
                               currency: &str,
                               show_description: &bool) {
    let mut transactions_table = String::new();

    if *show_description {
        transactions_table.push_str(&format!("row\tdate\tcounterparty\tamount \
                                              ({})\tdescription\n",
                                             currency));
        for (idx, transaction) in transactions.iter().enumerate() {
            let row_number = (idx + 1) as u32;
            let new_transaction_row = format!("{}\t{}\t{}\t{}\t{}\n",
                                              row_number,
                                              transaction.date,
                                              transaction.counterparty,
                                              transaction.amount,
                                              transaction.description);
            transactions_table = transactions_table + &new_transaction_row;
        }
    } else {
        transactions_table.push_str(&format!("row\tdate\tcounterparty\tamount ({})\n", currency));
        for (idx, transaction) in transactions.iter().enumerate() {
            let row_number = (idx + 1) as u32;
            let new_transaction_row = format!("{}\t{}\t{}\t{}\n",
                                              row_number,
                                              transaction.date,
                                              transaction.counterparty,
                                              transaction.amount);
            transactions_table = transactions_table + &new_transaction_row;
        }
    }

    let transactions_str = to_aligned_table(&transactions_table);

    print!("{}", transactions_str)
}

pub fn list_transactions_command(teller: &TellerClient,
                                 config: &Config,
                                 account: &AccountType,
                                 timeframe: &Timeframe,
                                 show_description: &bool)
                                 -> i32 {
    info!("Calling the list transactions command");
    let account_id = config.get_account_id(&account);
    let (from, to) = timeframe_to_date_range(&timeframe);
    teller.get_transactions_with_currency(&account_id, &from, &to)
        .map(|transactions_with_currency| {
            let TransactionsWithCurrrency { transactions, currency } = transactions_with_currency;
            represent_list_transactions(&transactions, &currency, &show_description);
            0
        })
        .unwrap_or_else(|err| {
            error!("Unable to list transactions: {}", err);
            1
        })
}
