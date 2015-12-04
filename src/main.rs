extern crate docopt;
extern crate rustc_serialize;
extern crate hyper;

mod config;
mod client;
mod inquirer;

use client::{get_accounts};
use config::{Config, read_config, write_config};
use inquirer::ask_question;

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};

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

    ask_question();

    let auth_token = String::new();
    let new_config = Config::new(auth_token);

    println!("fake token: {}", new_config.auth_token);

    let _ = write_config(&new_config);

    ()
}
