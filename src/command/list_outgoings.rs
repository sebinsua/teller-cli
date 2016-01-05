use config::Config;
use api::TellerClient;
use api::inform::{Outgoings, GetOutgoings};
use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

use super::representations::represent_list_amounts;

fn represent_list_outgoings(hac: &Outgoings, output: &OutputFormat) {
    represent_list_amounts("outgoing", &hac, &output)
}

pub fn list_outgoings_command(config: &Config,
                              account: &AccountType,
                              interval: &Interval,
                              timeframe: &Timeframe,
                              output: &OutputFormat)
                              -> i32 {
    info!("Calling the list outgoings command");
    let account_id = config.get_account_id(&account);
    let teller = TellerClient::new(&config.auth_token);
    teller.get_outgoings(&account_id, &interval, &timeframe)
          .map(|outgoings| {
              represent_list_outgoings(&outgoings, &output);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to list outgoings: {}", err);
              1
          })
}
