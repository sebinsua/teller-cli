extern crate rustc_serialize;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FormatResult};

use hyper::error::Error as HttpError;
use std::io::Error as StdIoError;
use rustc_serialize::json::DecoderError;

#[derive(Debug)]
pub enum TellerClientError {
    AuthenticationError,
    HttpClientError(HttpError),
    IoError(StdIoError),
    JsonParseError(DecoderError),
}

impl Display for TellerClientError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        self.description().fmt(f)
    }
}

impl From<HttpError> for TellerClientError {
    fn from(e: HttpError) -> TellerClientError {
        TellerClientError::HttpClientError(e)
    }
}

impl From<StdIoError> for TellerClientError {
    fn from(e: StdIoError) -> TellerClientError {
        TellerClientError::IoError(e)
    }
}

impl From<rustc_serialize::json::DecoderError> for TellerClientError {
    fn from(e: rustc_serialize::json::DecoderError) -> TellerClientError {
        TellerClientError::JsonParseError(e)
    }
}

impl StdError for TellerClientError {
    fn description(&self) -> &str {
        match *self {
            TellerClientError::AuthenticationError => "Could not authenticate",
            TellerClientError::HttpClientError(ref err) => err.description(),
            TellerClientError::IoError(ref err) => err.description(),
            TellerClientError::JsonParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            TellerClientError::HttpClientError(ref err) => err.cause(),
            TellerClientError::IoError(ref err) => err.cause(),
            TellerClientError::JsonParseError(ref err) => err.cause(),
            _ => None,
        }
    }
}
