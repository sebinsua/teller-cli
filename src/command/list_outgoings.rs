use config::Config;
use api::TellerClient;
use api::inform::{Outgoings, GetOutgoings};
use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

use command::representations::represent_list_amounts;

use command::timeframe_to_date_range;

fn represent_list_outgoings(hac: &Outgoings, output: &OutputFormat) {
    represent_list_amounts("outgoing", &hac, &output)
}

pub fn list_outgoings_command(teller: &TellerClient,
                              config: &Config,
                              account: &AccountType,
                              interval: &Interval,
                              timeframe: &Timeframe,
                              output: &OutputFormat)
                              -> i32 {
    info!("Calling the list outgoings command");
    let account_id = config.get_account_id(&account);
    let (from, to) = timeframe_to_date_range(&timeframe);
    teller.get_outgoings(&account_id, &interval, &from, &to)
          .map(|outgoings| {
              represent_list_outgoings(&outgoings, &output);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to list outgoings: {}", err);
              1
          })
}
