use config::Config;
use api::TellerClient;
use api::inform::{Incomings, GetIncomings};
use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

use command::representations::represent_list_amounts;

use command::timeframe_to_date_range;

fn represent_list_incomings(hac: &Incomings, output: &OutputFormat) {
    represent_list_amounts("incoming", &hac, &output)
}

pub fn list_incomings_command(teller: &TellerClient,
                              config: &Config,
                              account: &AccountType,
                              interval: &Interval,
                              timeframe: &Timeframe,
                              output: &OutputFormat)
                              -> i32 {
    info!("Calling the list incomings command");
    let account_id = config.get_account_id(&account);
    let (from, to) = timeframe_to_date_range(&timeframe);
    teller.get_incomings(&account_id, &interval, &from, &to)
          .map(|incomings| {
              represent_list_incomings(&incomings, &output);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to list incomings: {}", err);
              1
          })
}
