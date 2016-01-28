use config::Config;
use api::TellerClient;
use api::inform::{Balances, GetBalances};
use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

use command::representations::represent_list_amounts;

use command::timeframe_to_date_range;

fn represent_list_balances(hac: &Balances, output: &OutputFormat) {
    represent_list_amounts("balance", &hac, &output)
}

pub fn list_balances_command(teller: &TellerClient,
                             config: &Config,
                             account: &AccountType,
                             interval: &Interval,
                             timeframe: &Timeframe,
                             output: &OutputFormat)
                             -> i32 {
    info!("Calling the list balances command");
    let account_id = config.get_account_id(&account);
    let (from, to) = timeframe_to_date_range(&timeframe);
    teller.get_balances(&account_id, &interval, &from, &to)
          .map(|balances| {
              represent_list_balances(&balances, &output);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to list balances: {}", err);
              1
          })
}
