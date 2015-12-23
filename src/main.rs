#[macro_use]
extern crate log;
extern crate env_logger;

extern crate docopt;
extern crate rustc_serialize;
extern crate chrono;
extern crate hyper;
extern crate tabwriter;
extern crate itertools;

mod config;
mod client;
mod inquirer;

use client::{Account, Transaction, Money, HistoricalAmountsWithCurrency, Balances, Outgoings, Incomings, get_accounts, get_account_balance, get_transactions_with_currency, get_balances, get_outgoings, get_incomings, get_outgoing, get_incoming};
use client::{Interval, Timeframe};

use std::path::PathBuf;
use config::{Config, get_config_path, get_config_file, read_config, get_config_file_to_write, write_config};

use inquirer::{Question, Answer, ask_question};

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};

use std::io::Write;
use tabwriter::TabWriter;
use std::process::exit;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "Banking for the command line.

Usage:
    teller init
    teller [list] accounts
    teller [list] transactions [<account> --timeframe=<tf> --show-description]
    teller [list] (balances|outgoings|incomings) [<account> --interval=<itv> --timeframe=<tf> --output=<of>]
    teller [show] balance [<account> --hide-currency]
    teller [show] outgoing [<account> --hide-currency]
    teller [show] incoming [<account> --hide-currency]
    teller [--help | --version]

Commands:
    init                    Configure.
    list accounts           List accounts.
    list transactions       List transactions.
    list balances           List balances during a timeframe.
    list outgoings          List outgoings during a timeframe.
    list incomings          List incomings during a timeframe.
    show balance            Show the current balance.
    show outgoing           Show the current outgoing.
    show incoming           Show the current incoming.

    NOTE: By default commands are applied to the 'current' <account>.

Options:
    -h --help               Show this screen.
    -V --version            Show version.
    -i --interval=<itv>     Group by an interval of time (default: monthly).
    -t --timeframe=<tf>     Operate upon a named period of time (default: 6-months).
    -d --show-description   Show descriptions against transactions.
    -c --hide-currency      Show money without currency codes.
    -o --output=<of>        Output in a particular format (e.g. spark).
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_init: bool,
    cmd_list: bool,
    cmd_show: bool,
    cmd_accounts: bool,
    cmd_transactions: bool,
    cmd_balances: bool,
    cmd_outgoings: bool,
    cmd_incomings: bool,
    cmd_balance: bool,
    cmd_outgoing: bool,
    cmd_incoming: bool,
    arg_account: AccountType,
    flag_interval: Interval,
    flag_timeframe: Timeframe,
    flag_show_description: bool,
    flag_hide_currency: bool,
    flag_output: OutputFormat,
    flag_help: bool,
    flag_version: bool,
}

#[derive(Debug)]
enum AccountType {
    Current,
    Savings,
    Business,
    Unknown(String),
    None
}

impl Decodable for AccountType {
    fn decode<D: Decoder>(d: &mut D) -> Result<AccountType, D::Error> {
        let s = try!(d.read_str());
        let default_acccount_type = AccountType::None;
        Ok(match &*s {
            "" => default_acccount_type,
            "current" => AccountType::Current,
            "savings" => AccountType::Savings,
            "business" => AccountType::Business,
            s => AccountType::Unknown(s.to_string()),
        })
    }
}

impl Decodable for Interval {
    fn decode<D: Decoder>(d: &mut D) -> Result<Interval, D::Error> {
        let s = try!(d.read_str());
        let default_interval = Interval::Monthly;
        Ok(match &*s {
            "" => default_interval,
            "monthly" => Interval::Monthly,
            _ => {
                error!("teller-cli currently only suports an interval of monthly");
                default_interval
            },
        })
    }
}

impl Decodable for Timeframe {
    fn decode<D: Decoder>(d: &mut D) -> Result<Timeframe, D::Error> {
        let s = try!(d.read_str());
        let default_timeframe = Timeframe::SixMonths;
        Ok(match &*s {
            "year" => Timeframe::Year,
            "6-months" => Timeframe::SixMonths,
            "3-months" => Timeframe::ThreeMonths,
            _ => default_timeframe,
        })
    }
}

#[derive(Debug)]
enum OutputFormat {
    Spark,
    Standard,
}

impl Decodable for OutputFormat {
    fn decode<D: Decoder>(d: &mut D) -> Result<OutputFormat, D::Error> {
        let s = try!(d.read_str());
        let default_output_format = OutputFormat::Standard;
        Ok(match &*s {
            "spark" => OutputFormat::Spark,
            "standard" => OutputFormat::Standard,
            _ => default_output_format,
        })
    }
}

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

fn configure_cli(config_file_path: &PathBuf) -> Option<Config> {
    match init_config() {
        None => None,
        Some(config) => {
            match get_config_file_to_write(&config_file_path) {
                Ok(mut config_file) => {
                    let _ = write_config(&mut config_file, &config);
                    Some(config)
                },
                Err(e) => panic!("ERROR: opening file to write: {}", e),
            }
        },
    }
}

fn init_config() -> Option<Config> {
    let get_auth_token_question = Question::new(
        "auth_token",
        "What is your `auth_token` on teller.io?",
    );

    let auth_token_answer = ask_question(&get_auth_token_question);

    let mut config = Config::new_with_auth_token_only(auth_token_answer.value);

    print!("\n");
    let accounts = match get_accounts(&config) {
        Ok(accounts) => accounts,
        Err(e) => panic!("Unable to list accounts: {}", e),
    };
    represent_list_accounts(&accounts, &config);

    println!("Please type the row (e.g. 3) of the account you wish to place against an alias and press <enter> to set this in the config. Leave empty if irrelevant.");
    print!("\n");

    let questions = vec![
        Question::new(
            "current",
            "Which is your current account?",
        ),
        Question::new(
            "savings",
            "Which is your savings account?",
        ),
        Question::new(
            "business",
            "Which is your business account?",
        ),
    ];

    let answers: Vec<Answer> = questions.iter().map(ask_question).collect();
    let non_empty_answers: Vec<&Answer> = answers.iter().filter(|&answer| !answer.value.is_empty()).collect();
    let mut fa_iter = non_empty_answers.iter();

    match fa_iter.find(|&answer| answer.name == "current") {
        None => (),
        Some(answer) => {
            let row_number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.current = accounts[(row_number - 1) as usize].id.to_owned()
        },
    };
    match fa_iter.find(|&answer| answer.name == "savings") {
        None => (),
        Some(answer) => {
            let row_number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.savings = accounts[(row_number - 1) as usize].id.to_owned()
        }
    };
    match fa_iter.find(|&answer| answer.name == "business") {
        None => (),
        Some(answer) => {
            let row_number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.business = accounts[(row_number - 1) as usize].id.to_owned()
        }
    };

    if config.auth_token.is_empty() {
        error!("`auth_token` was invalid so a config could not be created");
        None
    } else {
        Some(config)
    }
}

fn pick_command(arguments: Args) {
    match arguments {
        Args { cmd_init, .. } if cmd_init == true => {
            let config_file_path = get_config_path();
            println!("To create the config ({}) we need to find out your `auth_token` and assign aliases to some common bank accounts.", config_file_path.display());
            print!("\n");
            configure_cli(&config_file_path);
            ()
        },
        Args { cmd_accounts, .. } if cmd_accounts == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                },
                Some(config) => list_accounts(&config),
            }
        },
        Args { cmd_balance, ref arg_account, flag_hide_currency, .. } if cmd_balance == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => show_balance(&config, &arg_account, &flag_hide_currency),
            }
        },
        Args { cmd_outgoing, ref arg_account, flag_hide_currency, .. } if cmd_outgoing == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => show_outgoing(&config, &arg_account, &flag_hide_currency),
            }
        },
        Args { cmd_incoming, ref arg_account, flag_hide_currency, .. } if cmd_incoming == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => show_incoming(&config, &arg_account, &flag_hide_currency),
            }
        },
        Args { cmd_transactions, ref arg_account, flag_show_description, ref flag_timeframe, .. } if cmd_transactions == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => list_transactions(&config, &arg_account, &flag_timeframe, &flag_show_description),
            }
        },
        Args { cmd_balances, ref arg_account, ref flag_interval, ref flag_timeframe, ref flag_output, .. } if cmd_balances == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => list_balances(&config, &arg_account, &flag_interval, &flag_timeframe, &flag_output),
            }
        },
        Args { cmd_incomings, ref arg_account, ref flag_interval, ref flag_timeframe, ref flag_output, .. } if cmd_incomings == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => list_incomings(&config, &arg_account, &flag_interval, &flag_timeframe, &flag_output),
            }
        },
        Args { cmd_outgoings, ref arg_account, ref flag_interval, ref flag_timeframe, ref flag_output, .. } if cmd_outgoings == true => {
            match get_config() {
                None => {
                    error!("Configuration could not be found or created so command not executed");
                    exit(1)
                }
                Some(config) => list_outgoings(&config, &arg_account, &flag_interval, &flag_timeframe, &flag_output),
            }
        },
        Args { flag_help, flag_version, .. } if flag_help == true || flag_version == true => (),
        _ => println!("{}", USAGE),
    }
}

fn get_account_alias_for_id<'a>(account_id: &str, config: &Config) -> &'a str {
    if *account_id == config.current {
        "(current)"
    } else if *account_id == config.savings {
        "(savings)"
    } else if *account_id == config.business {
        "(business)"
    } else {
        ""
    }
}

fn represent_list_accounts(accounts: &Vec<Account>, config: &Config) {
    let mut accounts_table = String::new();
    accounts_table.push_str("row\taccount no.\tbalance\n");
    for (idx, account) in accounts.iter().enumerate() {
        let row_number = (idx + 1) as u32;
        let account_alias = get_account_alias_for_id(&account.id, &config);
        let new_account_row = format!("{} {}\t****{}\t{}\t{}\n", row_number, account_alias, account.account_number_last_4, account.balance, account.currency);
        accounts_table = accounts_table + &new_account_row;
    }

    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", accounts_table).unwrap();
    tw.flush().unwrap();

    let accounts_str = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", accounts_str)
}

fn list_accounts(config: &Config) {
    match get_accounts(&config) {
        Ok(accounts) => represent_list_accounts(&accounts, &config),
        Err(e) => {
            error!("Unable to list accounts: {}", e);
            exit(1)
        },
    }
}

fn represent_money(balance_with_currency: &Money, hide_currency: &bool) {
    println!("{}", balance_with_currency.get_balance_for_display(&hide_currency))
}

fn get_account_id(config: &Config, account: &AccountType) -> String{
    let default_account_id = config.current.to_owned();
    match *account {
        AccountType::Current => config.current.to_owned(),
        AccountType::Savings => config.savings.to_owned(),
        AccountType::Business => config.business.to_owned(),
        _ => default_account_id,
    }
}

fn show_balance(config: &Config, account: &AccountType, hide_currency: &bool) {
    let account_id = get_account_id(&config, &account);
    match get_account_balance(&config, &account_id) {
        Ok(balance) => represent_money(&balance, &hide_currency),
        Err(e) => {
            error!("Unable to get account balance: {}", e);
            exit(1)
        },
    }
}

fn show_outgoing(config: &Config, account: &AccountType, hide_currency: &bool) {
    let account_id = get_account_id(&config, &account);
    match get_outgoing(&config, &account_id) {
        Ok(outgoing) => represent_money(&outgoing, &hide_currency),
        Err(e) => {
            error!("Unable to get outgoing: {}", e);
            exit(1)
        },
    }
}

fn show_incoming(config: &Config, account: &AccountType, hide_currency: &bool) {
    let account_id = get_account_id(&config, &account);
    match get_incoming(&config, &account_id) {
        Ok(incoming) => represent_money(&incoming, &hide_currency),
        Err(e) => {
            error!("Unable to get incoming: {}", e);
            exit(1)
        },
    }
}

fn represent_list_transactions(transactions: &Vec<Transaction>, currency: &str, show_description: &bool) {
    let mut transactions_table = String::new();

    if *show_description {
        transactions_table.push_str(&format!("row\tdate\tcounterparty\tamount ({})\tdescription\n", currency));
        for (idx, transaction) in transactions.iter().enumerate() {
            let row_number = (idx + 1) as u32;
            let new_transaction_row = format!("{}\t{}\t{}\t{}\t{}\n", row_number, transaction.date, transaction.counterparty, transaction.amount, transaction.description);
            transactions_table = transactions_table + &new_transaction_row;
        }
    } else {
        transactions_table.push_str(&format!("row\tdate\tcounterparty\tamount ({})\n", currency));
        for (idx, transaction) in transactions.iter().enumerate() {
            let row_number = (idx + 1) as u32;
            let new_transaction_row = format!("{}\t{}\t{}\t{}\n", row_number, transaction.date, transaction.counterparty, transaction.amount);
            transactions_table = transactions_table + &new_transaction_row;
        }
    }

    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", transactions_table).unwrap();
    tw.flush().unwrap();

    let transactions_str = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", transactions_str)
}

fn list_transactions(config: &Config, account: &AccountType, timeframe: &Timeframe, show_description: &bool) {
    let account_id = get_account_id(&config, &account);
    match get_transactions_with_currency(&config, &account_id, &timeframe) {
        Ok(transactions_with_currency) => represent_list_transactions(&transactions_with_currency.transactions, &transactions_with_currency.currency, &show_description),
        Err(e) => {
            error!("Unable to list transactions: {}", e);
            exit(1)
        },
    }
}

fn represent_list_amounts(amount_type: &str, hac: &HistoricalAmountsWithCurrency, output: &OutputFormat) {
    match *output {
        OutputFormat::Spark => {
            let balance_str = hac.historical_amounts.iter().map(|b| b.1.to_owned()).collect::<Vec<String>>().join(" ");
            println!("{}", balance_str)
        },
        OutputFormat::Standard => {
            let mut hac_table = String::new();
            let month_cols = hac.historical_amounts.iter().map(|historical_amount| historical_amount.0.to_owned()).collect::<Vec<String>>().join("\t");
            hac_table.push_str(&format!("\t{}\n", month_cols));
            hac_table.push_str(&format!("{} ({})", amount_type, hac.currency));
            for historical_amount in hac.historical_amounts.iter() {
                let new_amount = format!("\t{}", historical_amount.1);
                hac_table = hac_table + &new_amount;
            }

            let mut tw = TabWriter::new(Vec::new());
            write!(&mut tw, "{}", hac_table).unwrap();
            tw.flush().unwrap();

            let hac_str = String::from_utf8(tw.unwrap()).unwrap();

            println!("{}", hac_str)
        },
    }
}

fn represent_list_balances(hac: &Balances, output: &OutputFormat) {
    represent_list_amounts("balance", &hac, &output)
}

fn list_balances(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) {
    let account_id = get_account_id(&config, &account);
    match get_balances(&config, &account_id, &interval, &timeframe) {
        Ok(balances) => represent_list_balances(&balances, &output),
        Err(e) => {
            error!("Unable to list balances: {}", e);
            exit(1)
        },
    }
}

fn represent_list_outgoings(hac: &Outgoings, output: &OutputFormat) {
    represent_list_amounts("outgoing", &hac, &output)
}

fn list_outgoings(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) {
    let account_id = get_account_id(&config, &account);
    match get_outgoings(&config, &account_id, &interval, &timeframe) {
        Ok(outgoings) => represent_list_outgoings(&outgoings, &output),
        Err(e) => {
            error!("Unable to list ougoings: {}", e);
            exit(1)
        },
    }
}

fn represent_list_incomings(hac: &Incomings, output: &OutputFormat) {
    represent_list_amounts("incoming", &hac, &output)
}

fn list_incomings(config: &Config, account: &AccountType, interval: &Interval, timeframe: &Timeframe, output: &OutputFormat) {
    let account_id = get_account_id(&config, &account);
    match get_incomings(&config, &account_id, &interval, &timeframe) {
        Ok(incomings) => represent_list_incomings(&incomings, &output),
        Err(e) => {
            error!("Unable to list incomings: {}", e);
            exit(1)
        },
    }
}

fn main() {
    env_logger::init().unwrap();

    let arguments: Args = Docopt::new(USAGE)
        .and_then(|d| {
            d.version(VERSION.map(|v| v.to_string()))
             .decode()
        })
        .unwrap_or_else(|e| e.exit());

    pick_command(arguments)
}
