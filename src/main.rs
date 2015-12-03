extern crate rustc_serialize;
extern crate docopt;

extern crate hyper;

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};

use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

use hyper::Client;
use hyper::header::{Authorization, Bearer};

// TODO: This is only temporary...
const TOKEN: &'static str = "";

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

#[derive(Debug, RustcDecodable)]
struct AccountResponse {
    data: Account,
}

#[derive(Debug, RustcDecodable)]
struct AccountsResponse {
    data: Vec<Account>,
}

#[derive(Debug, RustcDecodable)]
struct Account {
    updated_at: String,
    institution: String,
    id: String,
    currency: String,
    balance: String,
    account_number_last_4: String,
}
// TODO: Maybe will need to deserialize with this:
// http://valve.github.io/blog/2014/08/26/json-serialization-in-rust-part-2/

// TODO: Work out when to use matching
//       and when to use try! and when to
//       use something else.

#[derive(Debug)]
enum ConfigError {
    IoError(io::Error),
    JsonParseError(rustc_serialize::json::DecoderError),
    JsonStringifyError(rustc_serialize::json::EncoderError),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fake display")
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> ConfigError {
        ConfigError::IoError(e)
    }
}

impl From<rustc_serialize::json::DecoderError> for ConfigError {
    fn from(e: rustc_serialize::json::DecoderError) -> ConfigError {
        ConfigError::JsonParseError(e)
    }
}

impl From<rustc_serialize::json::EncoderError> for ConfigError {
    fn from(e: rustc_serialize::json::EncoderError) -> ConfigError {
        ConfigError::JsonStringifyError(e)
    }
}

impl std::error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::IoError(_) => "I/O Error",
            ConfigError::JsonParseError(_) => "JSON parsing error",
            ConfigError::JsonStringifyError(_) => "JSON stringify error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ConfigError::IoError(ref err) => Some(err as &std::error::Error),
            ConfigError::JsonParseError(ref err) => Some(err as &std::error::Error),
            ConfigError::JsonStringifyError(ref err) => Some(err as &std::error::Error),
        }
    }
}

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

// TODO: Turns out you've been polluting the namespace really really badly
//       and you're gonna need to fix that!
#[derive(Debug)]
enum ApiServiceError {
    HttpClientError(hyper::error::Error),
    IoError(io::Error),
    JsonParseError(rustc_serialize::json::DecoderError),
}

impl std::fmt::Display for ApiServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fake display")
    }
}

impl From<hyper::error::Error> for ApiServiceError {
    fn from(e: hyper::error::Error) -> ApiServiceError {
        ApiServiceError::HttpClientError(e)
    }
}

impl From<io::Error> for ApiServiceError {
    fn from(e: io::Error) -> ApiServiceError {
        ApiServiceError::IoError(e)
    }
}

impl From<rustc_serialize::json::DecoderError> for ApiServiceError {
    fn from(e: rustc_serialize::json::DecoderError) -> ApiServiceError {
        ApiServiceError::JsonParseError(e)
    }
}

impl std::error::Error for ApiServiceError {
    fn description(&self) -> &str {
        match *self {
            ApiServiceError::HttpClientError(_) => "Api Error",
            ApiServiceError::IoError(_) => "I/O Error",
            ApiServiceError::JsonParseError(_) => "JSON parsing error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ApiServiceError::HttpClientError(ref err) => Some(err as &std::error::Error),
            ApiServiceError::IoError(ref err) => Some(err as &std::error::Error),
            ApiServiceError::JsonParseError(ref err) => Some(err as &std::error::Error),
        }
    }
}

type ApiServiceResult<T> = Result<T, ApiServiceError>;

fn get_accounts() -> ApiServiceResult<AccountsResponse> {
    let client = Client::new();

    let auth_header = Authorization(
        Bearer {
            token: TOKEN.to_owned()
        }
    );

    let mut res = try!(client.get("https://api.teller.io/accounts")
                             .header(auth_header)
                             .send()
                             .and_then(|r| {
                                 if r.status.is_client_error() {
                                     Err(hyper::error::Error::Status)
                                 } else {
                                     Ok(r)
                                 }
                             }));

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    println!("Response: {}", body);
    let accounts_response = try!(json::decode(&body));

    Ok(accounts_response)
}

#[allow(dead_code)]
fn get_account() -> ApiServiceResult<AccountResponse> {
    let client = Client::new();

    let auth_header = Authorization(
        Bearer {
            token: TOKEN.to_owned()
        }
    );

    let mut res = try!(client.get("https://api.teller.io/accounts/4803f712-cc3e-4560-9f80-3be8116d7723")
                             .header(auth_header)
                             .send()
                             .and_then(|r| {
                                if r.status.is_client_error() {
                                    Err(hyper::error::Error::Status)
                                } else {
                                    Ok(r)
                                }
                            }));

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    println!("Response: {}", body);
    let account_response = try!(json::decode(&body));

    Ok(account_response)
}

#[allow(dead_code)]
fn get_account_balance() -> ApiServiceResult<String> {
    get_account().map(|r| r.data.balance)
}

fn main() {
    // TODO: Currently this gets data but it will panic! if no JSON comes back, for example if we
    //       get a 500 server error. (There is no `error` property bizarrely.)
    match get_accounts() {
        Ok(_) => println!("dont print value "),
        Err(why) => println!("error: {}", std::error::Error::description(&why)),
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
        Err(why) => panic!("config could not be read"),
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
