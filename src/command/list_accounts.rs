use config::Config;
use client::get_accounts;

use super::representations::represent_list_accounts;

pub fn list_accounts_command(config: &Config) -> i32 {
    info!("Calling the list accounts command");
    get_accounts(&config)
        .map(|accounts| {
            represent_list_accounts(&accounts, &config);
            0
        })
        .unwrap_or_else(|err| {
            error!("Unable to list accounts: {}", err);
            1
        })
}
