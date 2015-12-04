extern crate rustc_serialize;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FormatResult};

use std::io::Error as StdIoError;
use rustc_serialize::json::{EncoderError, DecoderError};

#[derive(Debug)]
pub enum ConfigError {
    IoError(StdIoError),
    JsonParseError(DecoderError),
    JsonStringifyError(EncoderError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        self.description().fmt(f)
    }
}

impl From<StdIoError> for ConfigError {
    fn from(e: StdIoError) -> ConfigError {
        ConfigError::IoError(e)
    }
}

impl From<DecoderError> for ConfigError {
    fn from(e: DecoderError) -> ConfigError {
        ConfigError::JsonParseError(e)
    }
}

impl From<EncoderError> for ConfigError {
    fn from(e: EncoderError) -> ConfigError {
        ConfigError::JsonStringifyError(e)
    }
}

impl StdError for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::IoError(ref err) => err.description(),
            ConfigError::JsonParseError(ref err) => err.description(),
            ConfigError::JsonStringifyError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            ConfigError::IoError(ref err) => err.cause(),
            ConfigError::JsonParseError(ref err) => err.cause(),
            ConfigError::JsonStringifyError(ref err) => err.cause(),
        }
    }
}
