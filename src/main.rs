#[macro_use]
extern crate log;
extern crate env_logger;

extern crate docopt;
extern crate rustc_serialize;
extern crate chrono;
extern crate hyper;
extern crate tabwriter;
extern crate itertools;

mod cli;
mod command;
mod config;
mod inquirer;
mod client;

use docopt::Docopt;
use cli::get_command_type;
use command::execute;
use std::process;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "Banking for the command line.

Usage:
    teller init
    teller \
                             [list] accounts
    teller [list] transactions [<account> \
                             --timeframe=<tf> --show-description]
    teller [list] \
                             counterparties [<account> --timeframe=<tf> --count=<n>]
    teller \
                             [list] (balances|outgoings|incomings) [<account> --interval=<itv> \
                             --timeframe=<tf> --output=<of>]
    teller [show] balance [<account> \
                             --hide-currency]
    teller [show] outgoing [<account> \
                             --hide-currency]
    teller [show] incoming [<account> \
                             --hide-currency]
    teller [--help | --version]

Commands:
    init                    \
                             Configure.
    list accounts           List accounts.
    list \
                             transactions       List transactions.
    list counterparties     \
                             List outgoing amounts grouped by counterparties.
    list balances           \
                             List balances during a timeframe.
    list outgoings          List \
                             outgoings during a timeframe.
    list incomings          List \
                             incomings during a timeframe.
    show balance            Show the \
                             current balance.
    show outgoing           Show the current \
                             outgoing.
    show incoming           Show the current incoming.

    \
                             NOTE: By default commands are applied to the 'current' <account>.

\
                             Options:
    -h --help               Show this screen.
    -V \
                             --version            Show version.
    -i --interval=<itv>     Group \
                             by an interval of time [default: monthly].
    -t --timeframe=<tf>     \
                             Operate upon a named period of time [default: 6-months].
    -c \
                             --count=<n>          Only the top N elements [default: 10].
    -d \
                             --show-description   Show descriptions against transactions.
    -c \
                             --hide-currency      Show money without currency codes.
    -o \
                             --output=<of>        Output in a particular format (e.g. spark).
";

fn main() {
    env_logger::init().unwrap();

    let arguments = Docopt::new(USAGE)
                        .and_then(|d| {
                            d.version(VERSION.map(|v| v.to_string()))
                             .decode()
                        })
                        .unwrap_or_else(|e| e.exit());

    let command_type = get_command_type(&arguments);

    let return_code = execute(USAGE, &command_type, &arguments);

    process::exit(return_code)
}
