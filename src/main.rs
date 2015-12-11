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
use config::{Config, get_config_path, get_config_file, read_config, write_config};
use inquirer::{ask_question};

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

fn ready_config() -> Config {
    let config_file_path = get_config_path();
    match get_config_file(config_file_path) {
        None => {
            // let _ = write_config(&new_config);
            init_config();
            Config::new(
                "written-auth".to_string(),
                "current".to_string(),
                "savings".to_string(),
                "business".to_string(),
            )
        },
        Some(mut config_file) => {
            read_config(&mut config_file).unwrap()
        },
    }
}

fn init_config() {
    println!("initing config file");
    // list_accounts
    // ask_question();
}

fn pick_command(arguments: Args, config: &Config) {
    match arguments {
        Args { cmd_accounts, .. } if cmd_accounts == true => list_accounts(config),
        Args { cmd_balance, ref arg_account, .. } if cmd_balance == true => show_balance(config, &arg_account),
        Args { flag_help, flag_version, .. } if flag_help == true || flag_version == true => (),
        _ => println!("{}", USAGE),
    }
}

fn list_accounts(config: &Config) {
    let represent_list_accounts = |accounts: Vec<Account>| {
        let mut accounts_table = String::new();
        for (i, account) in accounts.iter().enumerate() {
            accounts_table = accounts_table + &format!("{}\t{}\t{} {}\n", (i + 1), account.account_number_last_4, account.balance, account.currency)[..];
        }

        let mut tw = TabWriter::new(Vec::new());
        write!(&mut tw, "{}", accounts_table).unwrap();
        tw.flush().unwrap();

        String::from_utf8(tw.unwrap()).unwrap()
    };

    let accounts = match get_accounts(&config) {
        Ok(accounts) => represent_list_accounts(accounts),
        Err(e) => panic!("Unable to list accounts: {}", e),
    };
    println!("{}", accounts);

    ()
}

fn show_balance(config: &Config, account: &AccountType) {
    let default_account_id = config.current.to_owned();
    let account_id = match *account {
        AccountType::Current => config.current.to_owned(),
        AccountType::Savings => config.savings.to_owned(),
        AccountType::Business => config.business.to_owned(),
        _ => default_account_id,
    };

    let balance = match get_account_balance(&config, account_id.to_string()) {
        Ok(balance) => balance,
        Err(e) => panic!("Unable to get account balance: {}", e),
    };
    println!("{}", balance);

    ()
}

fn main() {
    env_logger::init().unwrap();

    let arguments: Args = Docopt::new(USAGE)
        .and_then(|d| {
            d.version(VERSION.map(|v| v.to_string()))
             .decode()
        })
        .unwrap_or_else(|e| e.exit());

    let config = ready_config();

    pick_command(arguments, &config);

    ()
}
