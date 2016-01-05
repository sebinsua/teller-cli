use config::Config;
use api::TellerClient;
use api::inform::GetCounterparties;
use cli::arg_types::{AccountType, Timeframe};

use super::representations::to_aligned_table;

fn represent_list_counterparties(counterparties: &Vec<(String, String)>,
                                 currency: &str,
                                 count: &i64) {
    let mut counterparties_table = String::new();

    counterparties_table.push_str(&format!("row\tcounterparty\tamount ({})\n", currency));
    let skip_n = counterparties.len() - (*count as usize);
    for (idx, counterparty) in counterparties.iter().skip(skip_n).enumerate() {
        let row_number = (idx + 1) as u32;
        let new_counterparty_row = format!("{}\t{}\t{}\n",
                                           row_number,
                                           counterparty.0,
                                           counterparty.1);
        counterparties_table = counterparties_table + &new_counterparty_row;
    }

    let counterparties_str = to_aligned_table(&counterparties_table);

    println!("{}", counterparties_str)
}

pub fn list_counterparties_command(config: &Config,
                                   account: &AccountType,
                                   timeframe: &Timeframe,
                                   count: &i64)
                                   -> i32 {
    info!("Calling the list counterparties command");
    let account_id = config.get_account_id(&account);
    let teller = TellerClient::new(&config.auth_token);
    teller.get_counterparties(&account_id, &timeframe)
          .map(|counterparties_with_currency| {
              represent_list_counterparties(&counterparties_with_currency.counterparties,
                                            &counterparties_with_currency.currency,
                                            &count);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to list counterparties: {}", err);
              1
          })
}
