use config::Config;
use client::get_accounts;

use super::representations::represent_list_accounts;

pub fn list_accounts_command(config: &Config) -> i32 {
    match get_accounts(&config) {
        Ok(accounts) => {
            represent_list_accounts(&accounts, &config);
            0
        },
        Err(e) => {
            error!("Unable to list accounts: {}", e);
            1
        },
    }
}
