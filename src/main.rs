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

fn get_config_file(config_path: &PathBuf) -> File {
    let display = config_path.display();
    let f = match File::open(&config_path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    f
}

fn get_config(config_path: &PathBuf) -> Config {
    let mut f = get_config_file(config_path);

    let display = config_path.display();
    let mut content_str = String::new();
    match f.read_to_string(&mut content_str) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    }

    let config: Config = json::decode(&content_str).unwrap();
    return config;
}

fn set_config(config: Config) {

}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| {
                                d.version(VERSION.map(|v| v.to_string()))
                                 .decode()
                            })
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    let config_path: PathBuf = match env::home_dir() {
        Some(mut p) => {
            p.push(".tellerrc");
            p
        },
        None => panic!("Impossible to get your home directory!"),
    };

    let config = get_config(&config_path);
    println!("{}", config.auth_token);

    println!("What's the auth token?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("{}", input),
        Err(error) => println!("error: {}", error),
    };

    ()
}
