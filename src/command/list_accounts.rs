use config::Config;
use api::TellerClient;

use command::representations::represent_list_accounts;

pub fn list_accounts_command(teller: &TellerClient,
                             config: &Config)
                             -> i32 {
    info!("Calling the list accounts command");
    teller.get_accounts()
          .map(|accounts| {
              represent_list_accounts(&accounts, &config);
              0
          })
          .unwrap_or_else(|err| {
              error!("Unable to list accounts: {}", err);
              1
          })
}
