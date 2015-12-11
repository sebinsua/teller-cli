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

use client::{get_accounts};
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

#[allow(dead_code)]
fn represent() {
    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "
    Bruce Springsteen\tBorn to Run
    Bob Seger\tNight Moves
    Metallica\tBlack
    The Boss\tDarkness on the Edge of Town
    ").unwrap();
    tw.flush().unwrap();

    let written = String::from_utf8(tw.unwrap()).unwrap();
    println!("{}", written);
}

fn ready_config() -> Config {
    let config_file_path = get_config_path();
    match get_config_file(config_file_path) {
        None => {
            // let _ = write_config(&new_config);
            init_config();
            Config::new(String::new())
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
    println!("calling list accounts");
    /* match get_accounts() {
        Ok(_) => println!("dont print value "),
        Err(why) => println!("error: {}", why),
    } */
    ()
}

fn show_balance(config: &Config, account: &AccountType) {
    println!("calling show balance");
    match *account {
        AccountType::Current => (),
        AccountType::Savings => (),
        AccountType::Business => (),
        _ => (),
    }
    // get_account_balance();
    // represent();
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
