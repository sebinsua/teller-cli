mod representations;

mod show_usage;
mod initialise;
mod list_accounts;
mod show_balance;
mod show_outgoing;
mod show_incoming;
mod list_transactions;
mod list_counterparties;
mod list_balances;
mod list_outgoings;
mod list_incomings;

use cli::{CommandType, CliArgs};
use config::{Config, get_config_path, get_config_file, read_config};
use self::initialise::configure_cli;

use self::show_usage::show_usage_command;
use self::initialise::initialise_command;
use self::list_accounts::list_accounts_command;
use self::show_balance::show_balance_command;
use self::show_outgoing::show_outgoing_command;
use self::show_incoming::show_incoming_command;
use self::list_transactions::list_transactions_command;
use self::list_counterparties::list_counterparties_command;
use self::list_balances::list_balances_command;
use self::list_outgoings::list_outgoings_command;
use self::list_incomings::list_incomings_command;

fn get_config() -> Option<Config> {
    let config_file_path = get_config_path();
    match get_config_file(&config_file_path) {
        None => {
            println!("A config file could not be found at: {}", config_file_path.display());
            println!("You will need to set the `auth_token` and give aliases to your bank accounts");
            print!("\n");
            configure_cli(&config_file_path)
        },
        Some(mut config_file) => {
            match read_config(&mut config_file) {
                Ok(config) => Some(config),
                Err(e) => panic!("ERROR: attempting to read file {}: {}", config_file_path.display(), e),
            }
        },
    }
}

fn do_nothing_command() -> i32 {
    debug!("--help or --version were passed in so we are not going to execute anything more...");
    0
}

pub fn execute(usage: &str, command_type: &CommandType, arguments: &CliArgs) -> i32 {
    match *command_type {
        CommandType::None => do_nothing_command(),
        CommandType::ShowUsage => show_usage_command(usage),
        CommandType::Initialise => initialise_command(),
        _ => {
            match get_config() {
                None => {
                    error!("The command was not executed since a config could not be found or created");
                    1
                },
                Some(config) => {
                    match *command_type {
                        CommandType::ListAccounts => {
                            list_accounts_command(&config)
                        },
                        CommandType::ShowBalance => {
                            let CliArgs { ref arg_account, flag_hide_currency, .. } = *arguments;
                            show_balance_command(&config, &arg_account, &flag_hide_currency)
                        },
                        CommandType::ShowOutgoing => {
                            let CliArgs { ref arg_account, flag_hide_currency, .. } = *arguments;
                            show_outgoing_command(&config, &arg_account, &flag_hide_currency)
                        },
                        CommandType::ShowIncoming => {
                            let CliArgs { ref arg_account, flag_hide_currency, .. } = *arguments;
                            show_incoming_command(&config, &arg_account, &flag_hide_currency)
                        },
                        CommandType::ListTransactions => {
                            let CliArgs { ref arg_account, flag_show_description, ref flag_timeframe, .. } = *arguments;
                            list_transactions_command(&config, &arg_account, &flag_timeframe, &flag_show_description)
                        },
                        CommandType::ListCounterparties => {
                            let CliArgs { ref arg_account, ref flag_timeframe, flag_count, .. } = *arguments;
                            list_counterparties_command(&config, &arg_account, &flag_timeframe, &flag_count)
                        },
                        CommandType::ListBalances => {
                            let CliArgs { ref arg_account, ref flag_interval, ref flag_timeframe, ref flag_output, .. } = *arguments;
                            list_balances_command(&config, &arg_account, &flag_interval, &flag_timeframe, &flag_output)
                        },
                        CommandType::ListOutgoings => {
                            let CliArgs { ref arg_account, ref flag_interval, ref flag_timeframe, ref flag_output, .. } = *arguments;
                            list_outgoings_command(&config, &arg_account, &flag_interval, &flag_timeframe, &flag_output)
                        },
                        CommandType::ListIncomings => {
                            let CliArgs { ref arg_account, ref flag_interval, ref flag_timeframe, ref flag_output, .. } = *arguments;
                            list_incomings_command(&config, &arg_account, &flag_interval, &flag_timeframe, &flag_output)
                        },
                        _ => panic!("This shoult not be accessible"),
                    }
                },
            }
        },
    }
}
