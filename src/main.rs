extern crate rustc_serialize;
extern crate docopt;

extern crate hyper;

// TODO: Turns out you've been polluting the namespace really really badly
//       and you're gonna need to fix that!

mod config;
mod client;

use config::error::ConfigError;

use client::{get_accounts};

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};

use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Banking for the command line.

Usage:
    teller list accounts
    teller show balance <account>
    teller -h | --help
    teller -V | --version

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
enum AccountType { Current, Savings, Business, Unknown(String), None }

impl Decodable for AccountType {
    fn decode<D: Decoder>(d: &mut D) -> Result<AccountType, D::Error> {
        let s = try!(d.read_str());
        Ok(match &*s {
            "" => AccountType::None,
            "current" => AccountType::Current,
            "savings" => AccountType::Savings,
            "business" => AccountType::Business,
            s => AccountType::Unknown(s.to_string()),
        })
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Config {
    auth_token: String,
}

impl Config {
    pub fn new(auth_token: String) -> Config {
        Config {
            auth_token: auth_token,
        }
    }
}

// TODO: Technically this should probably be passed into the
//       functions that require it.
fn get_config_path() -> PathBuf {
    let fallback_config_path = PathBuf::from("./.tellerrc");
    let append_config_file = |mut p: PathBuf| {
        p.push(".tellerrc");
        p
    };
    env::home_dir().map_or(fallback_config_path, append_config_file)
}

fn get_config_file() -> Result<File, io::Error> {
    let config_path = get_config_path();
    File::open(&config_path)
}

fn get_config_file_to_write() -> Result<File, io::Error> {
    let config_path = get_config_path();
    File::create(&config_path)
}

fn read_config() -> Result<Config, ConfigError> {
    let mut config_file = try!(get_config_file());

    let mut content_str = String::new();
    try!(config_file.read_to_string(&mut content_str));

    let config: Config = try!(json::decode(&content_str));

    Ok(config)
}

fn write_config(config: &Config) -> Result<(), ConfigError> {
    let mut config_file = try!(get_config_file_to_write());

    let content_str = try!(json::encode(&config));

    try!(config_file.write_all(content_str.as_bytes()));

    Ok(())
}

fn main() {
    // TODO: Currently this gets data but it will panic! if no JSON comes back, for example if we
    //       get a 500 server error. (There is no `error` property bizarrely.)
    match get_accounts() {
        Ok(_) => println!("dont print value "),
        Err(why) => println!("error: {}", why),
    }

    // get_account_balance();

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| {
                                d.version(VERSION.map(|v| v.to_string()))
                                 .decode()
                            })
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    // TODO: If there is a config file then read it,
    //       otherwise ask a question and write to it.
    let config = match read_config() {
        Ok(config) => config,
        Err(why) => panic!(why),
    };
    println!("{}", config.auth_token);

    println!("What's the auth token?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("{}", input),
        Err(error) => println!("error: {}", error),
    };

    let auth_token = String::new();
    let new_config = Config::new(auth_token);

    println!("fake token: {}", new_config.auth_token);

    let _ = write_config(&new_config);

    ()
}
