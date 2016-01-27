pub mod error;

use rustc_serialize::json;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Error as StdIoError;
use std::io::ErrorKind;

use std::io::prelude::*; // Required for read_to_string use later.

use self::error::ConfigError;

use cli::arg_types::AccountType;

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
        Config::new(auth_token.into(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string())
    }

    pub fn get_account_id(&self, account: &AccountType) -> String {
        let default_account_id = self.current.to_owned();
        match *account {
            AccountType::Current => self.current.to_owned(),
            AccountType::Savings => self.savings.to_owned(),
            AccountType::Business => self.business.to_owned(),
            _ => default_account_id,
        }
    }

    pub fn get_account_alias_for_id<'a>(&self, account_id: &str) -> &'a str {
        if *account_id == self.current {
            "(current)"
        } else if *account_id == self.savings {
            "(savings)"
        } else if *account_id == self.business {
            "(business)"
        } else {
            ""
        }
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
    let config_path_str = config_path.to_str().unwrap_or("[error: config_path#to_str fails]");
    info!("Checking whether config file within {} exists",
          config_path_str);
    let config_file = File::open(&config_path);
    match config_file {
        Err(ref e) if ErrorKind::NotFound == e.kind() => {
            debug!("No config file found");
            None
        }
        Err(_) => panic!("Unable to read config!"),
        Ok(config_file) => Some(config_file),
    }
}

pub fn get_config_file_to_write(config_path: &PathBuf) -> Result<File, StdIoError> {
    let config_file = File::create(&config_path);
    match config_file {
        Err(ref e) if ErrorKind::PermissionDenied == e.kind() => {
            panic!("Permission to read config denied")
        }
        _ => config_file,
    }
}

pub fn get_config() -> Option<Config> {
    let config_file_path = get_config_path();
    match get_config_file(&config_file_path) {
        None => None,
        Some(mut config_file) => {
            match read_config(&mut config_file) {
                Ok(config) => Some(config),
                Err(e) => {
                    panic!("ERROR: attempting to read file {}: {}",
                           config_file_path.display(),
                           e)
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::Config;

    use cli::arg_types::AccountType;

    use std::env;
    use std::path::PathBuf;
    use super::get_config_path;

    #[test]
    fn can_instantiate_config() {
        let expected_auth_token = "fake-auth-token";
        let expected_current = "current-id";
        let expected_savings = "savings-id";
        let expected_business = "business-id";

        let config = Config::new(expected_auth_token, expected_current, expected_savings, expected_business);

        assert_eq!(expected_auth_token, config.auth_token);
        assert_eq!(expected_current, config.current);
        assert_eq!(expected_savings, config.savings);
        assert_eq!(expected_business, config.business);
    }

    #[test]
    fn can_get_account_id_from_config() {
        let expected_auth_token = "fake-auth-token";
        let expected_current = "current-id";
        let expected_savings = "savings-id";
        let expected_business = "business-id";

        let config = Config::new(expected_auth_token, expected_current, expected_savings, expected_business);

        assert_eq!("savings-id", config.get_account_id(&AccountType::Savings));
    }

    #[test]
    fn can_get_account_alias_from_config() {
        let expected_auth_token = "fake-auth-token";
        let expected_current = "current-id";
        let expected_savings = "savings-id";
        let expected_business = "business-id";

        let config = Config::new(expected_auth_token, expected_current, expected_savings, expected_business);

        assert_eq!("(savings)", config.get_account_alias_for_id(&expected_savings));
    }

    // test:
    //
    // read_config: Read trait
    // write_config: Write trait
    //
    // get_config_file and get_config_file_to_write can't be tested afaik.
    // get_config: looks like it's not possible to test without get_config_file.

    #[test]
    fn can_get_config_path() {
        let config_path = get_config_path();
        let home_dir = env::home_dir().unwrap_or(PathBuf::from("."));
        assert_eq!(format!("{}/.tellerrc", home_dir.display()), config_path.to_str().unwrap());
    }

}
