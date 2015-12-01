extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Banking for your command line.

Usage:
    teller show [ --current | --business ]
    teller -h | --help
    teller -V | --version

Options:
    -h --help       Show this screen.
    -V --version    Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_show: bool,
    flag_help: bool,
    flag_version: bool,
}

#[derive(RustcEncodable, RustcDecodable)]
struct Config {
    auth_token: String,
}

fn get_config() -> Config {
    let config_path: PathBuf = match env::home_dir() {
        Some(mut p) => {
            p.push(".tellerrc");
            p
        },
        None => panic!("Impossible to get your home directory!"),
    };

    let display = config_path.display();
    println!("{}", display);
    let mut f = match File::open(&config_path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut content_str = String::new();
    match f.read_to_string(&mut content_str) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    }

    let config: Config = json::decode(&content_str).unwrap();
    return config;
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| {
                                d.version(VERSION.map(|v| v.to_string()))
                                 .decode()
                            })
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    let config = get_config();
    println!("{}", config.auth_token);

    ();
}
