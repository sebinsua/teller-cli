pub mod error;

use rustc_serialize::json;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Error as StdIoError;

use std::io::prelude::*; // Required for read_to_string use later.

use self::error::ConfigError;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
    pub auth_token: String,
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

fn get_config_file() -> Result<File, StdIoError> {
    let config_path = get_config_path();
    File::open(&config_path)
}

fn get_config_file_to_write() -> Result<File, StdIoError> {
    let config_path = get_config_path();
    File::create(&config_path)
}

pub fn read_config() -> Result<Config, ConfigError> {
    let mut config_file = try!(get_config_file());

    let mut content_str = String::new();
    try!(config_file.read_to_string(&mut content_str));

    let config: Config = try!(json::decode(&content_str));

    Ok(config)
}

pub fn write_config(config: &Config) -> Result<(), ConfigError> {
    let mut config_file = try!(get_config_file_to_write());

    let content_str = try!(json::encode(&config));

    try!(config_file.write_all(content_str.as_bytes()));

    Ok(())
}
