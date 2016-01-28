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

use api::TellerClient;
use config::{Config, get_config, get_config_path};
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

use chrono::{Date, UTC};
use chrono::duration::Duration;
use cli::arg_types::Timeframe;
use api::client::generate_utc_date_from_date_str;

pub fn timeframe_to_date_range(timeframe: &Timeframe) -> (Date<UTC>, Date<UTC>){
    // NOTE: We need to ensure that when testing the from and to dates used
    // are always the same.
    let to = if cfg!(test) {
        generate_utc_date_from_date_str("2016-01-01")
    } else {
        UTC::today()
    };

    // This is not ideal. Might be off by a few days.
    let from = match *timeframe {
        Timeframe::ThreeMonths => to - Duration::days(91),
        Timeframe::SixMonths => to - Duration::days(183),
        Timeframe::Year => to - Duration::days(365),
    };

    (from, to)
}

fn ensure_config() -> Option<Config> {
    get_config().or_else(|| {
        let config_file_path = get_config_path();
        println!("A config file could not be found at: {}",
                 config_file_path.display());
        println!("You will need to set the `auth_token` and give aliases to your bank accounts");
        print!("\n");
        configure_cli(&config_file_path)
    })
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
            match ensure_config() {
                None => {
                    error!("The command was not executed since a config could not be found or \
                            generated");
                    1
                }
                Some(config) => {
                    let teller = TellerClient::new(&config.auth_token);
                    match *command_type {
                        CommandType::ListAccounts => list_accounts_command(&teller, &config),
                        CommandType::ShowBalance => {
                            let CliArgs { ref arg_account, flag_hide_currency, .. } = *arguments;
                            show_balance_command(&teller, &config, &arg_account, &flag_hide_currency)
                        }
                        CommandType::ShowOutgoing => {
                            let CliArgs { ref arg_account, flag_hide_currency, .. } = *arguments;
                            show_outgoing_command(&teller, &config, &arg_account, &flag_hide_currency)
                        }
                        CommandType::ShowIncoming => {
                            let CliArgs { ref arg_account, flag_hide_currency, .. } = *arguments;
                            show_incoming_command(&teller, &config, &arg_account, &flag_hide_currency)
                        }
                        CommandType::ListTransactions => {
                            let CliArgs {
                                ref arg_account,
                                flag_show_description,
                                ref flag_timeframe,
                                ..
                            } = *arguments;
                            list_transactions_command(&teller,
                                                      &config,
                                                      &arg_account,
                                                      &flag_timeframe,
                                                      &flag_show_description)
                        }
                        CommandType::ListCounterparties => {
                            let CliArgs {
                                ref arg_account,
                                ref flag_timeframe,
                                flag_count,
                                ..
                            } = *arguments;
                            list_counterparties_command(&teller,
                                                        &config,
                                                        &arg_account,
                                                        &flag_timeframe,
                                                        &flag_count)
                        }
                        CommandType::ListBalances => {
                            let CliArgs {
                                ref arg_account,
                                ref flag_interval,
                                ref flag_timeframe,
                                ref flag_output,
                                ..
                            } = *arguments;
                            list_balances_command(&teller,
                                                  &config,
                                                  &arg_account,
                                                  &flag_interval,
                                                  &flag_timeframe,
                                                  &flag_output)
                        }
                        CommandType::ListOutgoings => {
                            let CliArgs {
                                ref arg_account,
                                ref flag_interval,
                                ref flag_timeframe,
                                ref flag_output,
                                ..
                            } = *arguments;
                            list_outgoings_command(&teller,
                                                   &config,
                                                   &arg_account,
                                                   &flag_interval,
                                                   &flag_timeframe,
                                                   &flag_output)
                        }
                        CommandType::ListIncomings => {
                            let CliArgs {
                                ref arg_account,
                                ref flag_interval,
                                ref flag_timeframe,
                                ref flag_output,
                                ..
                            } = *arguments;
                            list_incomings_command(&teller,
                                                   &config,
                                                   &arg_account,
                                                   &flag_interval,
                                                   &flag_timeframe,
                                                   &flag_output)
                        }
                        _ => panic!("This should not have been executable but for some reason was"),
                    }
                }
            }
        }
    }
}
