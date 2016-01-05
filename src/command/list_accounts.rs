use config::Config;
use api::TellerClient;

use super::representations::represent_list_accounts;

pub fn list_accounts_command(config: &Config) -> i32 {
    info!("Calling the list accounts command");
    let teller = TellerClient::new(&config.auth_token);
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
