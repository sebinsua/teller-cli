#![allow(dead_code, unused_imports, unused_variables)]

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate docopt;
extern crate rustc_serialize;
extern crate hyper;
extern crate tabwriter;

mod config;
mod client;
mod inquirer;

use client::{Account, get_accounts, get_account_balance};
use config::{Config, get_config_path, get_config_file, read_config, get_config_file_to_write, write_config};
use inquirer::{Question, Answer, ask_question};

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};

use std::io::ErrorKind;

use std::io::Write;
use tabwriter::TabWriter;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Banking for the command line.

Usage:
    teller [list] accounts
    teller [show] balance [<account>]
    teller [--help | --version]

Commands:
    list accounts   List accounts.
    show balance    Show the balance of an account.

Options:
    -h --help       Show this screen.
    -V --version    Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_list: bool,
    cmd_accounts: bool,
    cmd_show: bool,
    cmd_balance: bool,
    arg_account: AccountType,
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

fn ready_config() -> Option<Config> {
    let config_file_path = get_config_path();
    match get_config_file(&config_file_path) {
        None => {
            println!("A config file could not be found at: {}", config_file_path.display());
            match init_config() {
                None => None,
                Some(config) => {
                    match get_config_file_to_write(&config_file_path) {
                        Ok(mut config_file) => {
                            write_config(&mut config_file, &config);
                            Some(config)
                        },
                        Err(e) => panic!("ERROR: opening file to write: {}", e),
                    }
                },
            }
        },
        Some(mut config_file) => {
            let config = read_config(&mut config_file).unwrap();
            Some(config)
        },
    }
}

fn init_config() -> Option<Config> {
    println!("You will need to set the `auth_token` and give aliases to your bank accounts");
    print!("\n");

    let get_auth_token_question = Question::new(
        "auth_token".to_string(),
        "What is your `auth_token` on teller.io?".to_string()
    );

    let auth_token_answer = ask_question(&get_auth_token_question);

    let mut config = Config::new(
        auth_token_answer.value,
        "".to_string(),
        "".to_string(),
        "".to_string(),
    );

    print!("\n");
    let accounts = match get_accounts(&config) {
        Ok(accounts) => accounts,
        Err(e) => panic!("Unable to list accounts: {}", e),
    };
    represent_list_accounts(&accounts);

    println!("Please type the id (e.g. 3) of the account you wish to place against an alias and press <enter> to set this in the config. Leave empty if irrelevant.");
    print!("\n");

    let questions = vec![
        Question::new(
            "current".to_string(),
            "Which is your current account?".to_string()
        ),
        Question::new(
            "savings".to_string(),
            "Which is your savings account?".to_string()
        ),
        Question::new(
            "business".to_string(),
            "Which is your business account?".to_string()
        ),
    ];

    let answers: Vec<Answer> = questions.iter().map(ask_question).collect();
    let filtered_answers: Vec<&Answer> = answers.iter().filter(|&answer| !answer.value.is_empty()).collect();
    let mut fa_iter = filtered_answers.iter();

    match fa_iter.find(|&answer| answer.name == "current") {
        None => (),
        Some(answer) => {
            let number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.current = accounts[(number - 1) as usize].id.to_owned()
        },
    };
    match fa_iter.find(|&answer| answer.name == "savings") {
        None => (),
        Some(answer) => {
            let number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.savings = accounts[(number - 1) as usize].id.to_owned()
        }
    };
    match fa_iter.find(|&answer| answer.name == "business") {
        None => (),
        Some(answer) => {
            let number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.business = accounts[(number - 1) as usize].id.to_owned()
        }
    };

    if config.auth_token.is_empty() {
        None
    } else {
        Some(config)
    }
}

fn pick_command(arguments: Args) {
    match arguments {
        Args { cmd_accounts, .. } if cmd_accounts == true => {
            match ready_config() {
                None => info!("Configuration could not be found or created"),
                Some(config) => list_accounts(&config),
            }
        },
        Args { cmd_balance, ref arg_account, .. } if cmd_balance == true => {
            match ready_config() {
                None => info!("Configuration could not be found or created"),
                Some(config) => show_balance(&config, &arg_account),
            }
        },
        Args { flag_help, flag_version, .. } if flag_help == true || flag_version == true => (),
        _ => println!("{}", USAGE),
    }
}

fn represent_list_accounts(accounts: &Vec<Account>) {
    let mut accounts_table = String::new();
    accounts_table.push_str("id\taccount no.\tbalance\n");
    for (i, account) in accounts.iter().enumerate() {
        accounts_table = accounts_table + &format!("{}\t****{}\t{}\t{}\n", (i + 1), account.account_number_last_4, account.balance, account.currency)[..];
    }

    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "{}", accounts_table).unwrap();
    tw.flush().unwrap();

    let accounts = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", accounts)
}

fn list_accounts(config: &Config) {
    match get_accounts(&config) {
        Ok(accounts) => represent_list_accounts(&accounts),
        Err(e) => panic!("Unable to list accounts: {}", e),
    }
}

fn represent_show_balance(balance: String) {
    println!("{}", balance);
}

fn show_balance(config: &Config, account: &AccountType) {
    let default_account_id = config.current.to_owned();
    let account_id = match *account {
        AccountType::Current => config.current.to_owned(),
        AccountType::Savings => config.savings.to_owned(),
        AccountType::Business => config.business.to_owned(),
        _ => default_account_id,
    };

    match get_account_balance(&config, account_id.to_string()) {
        Ok(balance) => represent_show_balance(balance),
        Err(e) => panic!("Unable to get account balance: {}", e),
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

    pick_command(arguments);

    ()
}
