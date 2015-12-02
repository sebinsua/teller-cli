extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

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
    arg_account: String,
    flag_help: bool,
    flag_version: bool,
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

// TODO: Work out when to use matching
//       and when to use try! and when to
//       use something else.

fn get_config_path() -> PathBuf {
    match env::home_dir() {
        Some(mut p) => {
            p.push(".tellerrc");
            p
        },
        None => panic!("Impossible to get your home directory!"),
    }
}

fn get_config_file() -> File {
    let config_path = get_config_path();

    match File::open(&config_path) {
        Err(why) => panic!("couldn't open {} to read: {}", config_path.display(), Error::description(&why)),
        Ok(file) => file,
    }
}

fn get_config_file_to_write() -> File {
    let config_path = get_config_path();

    match File::create(&config_path) {
        Err(why) => panic!("couldn't open {} to write: {}", config_path.display(), Error::description(&why)),
        Ok(file) => file,
    }
}

fn read_config() -> Config {
    let mut config_file = get_config_file();

    let mut content_str = String::new();
    match config_file.read_to_string(&mut content_str) {
        Err(why) => panic!("couldn't read: {}", Error::description(&why)),
        Ok(_) => (),
    }

    let config: Config = json::decode(&content_str).unwrap();
    return config;
}

fn write_config(config: &Config) {
    let mut config_file = get_config_file_to_write();

    let content_str = json::encode(&config).unwrap();

    config_file.write_all(content_str.as_bytes()).unwrap()
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| {
                                d.version(VERSION.map(|v| v.to_string()))
                                 .decode()
                            })
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    // TODO: If there is a config file then read it,
    //       otherwise ask a question and write to it.
    let config = read_config();
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

    write_config(&new_config);

    ()
}
