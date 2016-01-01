pub mod error;

use rustc_serialize::json;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Error as StdIoError;
use std::io::ErrorKind;

use std::io::prelude::*; // Required for read_to_string use later.

use self::error::ConfigError;

use cli::parse::AccountType;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
    pub auth_token: String,
    pub current: String,
    pub savings: String,
    pub business: String,
}

impl Config {
    pub fn new<S: Into<String>>(auth_token: S, current: S, savings: S, business: S) -> Config {
        Config {
            auth_token: auth_token.into(),
            current: current.into(),
            savings: savings.into(),
            business: business.into(),
        }
    }

    pub fn new_with_auth_token_only<S: Into<String>>(auth_token: S) -> Config {
        Config::new(
            auth_token.into(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        )
    }
}

pub fn get_config_path() -> PathBuf {
    let fallback_config_path = PathBuf::from("./.tellerrc");
    let append_config_file = |mut p: PathBuf| {
        p.push(".tellerrc");
        p
    };
    env::home_dir().map_or(fallback_config_path, append_config_file)
}

pub fn get_config_file(config_path: &PathBuf) -> Option<File> {
    info!("Checking whether config file within {} exists", config_path.to_str().unwrap());
    let config_file = File::open(&config_path);
    match config_file {
        Err(ref e) if ErrorKind::NotFound == e.kind() => {
            debug!("No config file found");
            None
        },
        Err(_) => panic!("Unable to read config!"),
        Ok(config_file) => Some(config_file),
    }
}

pub fn get_config_file_to_write(config_path: &PathBuf) -> Result<File, StdIoError> {
    let config_file = File::create(&config_path);
    match config_file {
        Err(ref e) if ErrorKind::PermissionDenied == e.kind() => panic!("Permission to read config denied"),
        _ => config_file,
    }
}

pub fn read_config(config_file: &mut File) -> Result<Config, ConfigError> {
    let mut content_str = String::new();
    try!(config_file.read_to_string(&mut content_str));

    debug!("config read as: {}", content_str);

    let config: Config = try!(json::decode(&content_str));

    Ok(config)
}

pub fn write_config(config_file: &mut File, config: &Config) -> Result<(), ConfigError> {
    let content_str = try!(json::encode(&config));

    try!(config_file.write_all(content_str.as_bytes()));

    Ok(())
}

pub fn get_account_id(config: &Config, account: &AccountType) -> String {
    let default_account_id = config.current.to_owned();
    match *account {
        AccountType::Current => config.current.to_owned(),
        AccountType::Savings => config.savings.to_owned(),
        AccountType::Business => config.business.to_owned(),
        _ => default_account_id,
    }
}
