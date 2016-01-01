use rustc_serialize::{Decodable, Decoder};

use super::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

#[derive(Debug, RustcDecodable)]
pub struct CliArgs {
    cmd_init: bool,
    cmd_list: bool,
    cmd_show: bool,
    cmd_accounts: bool,
    cmd_transactions: bool,
    cmd_counterparties: bool,
    cmd_balances: bool,
    cmd_outgoings: bool,
    cmd_incomings: bool,
    cmd_balance: bool,
    cmd_outgoing: bool,
    cmd_incoming: bool,
    pub arg_account: AccountType,
    pub flag_interval: Interval,
    pub flag_timeframe: Timeframe,
    pub flag_count: i64,
    pub flag_show_description: bool,
    pub flag_hide_currency: bool,
    pub flag_output: OutputFormat,
    flag_help: bool,
    flag_version: bool,
}

impl Decodable for AccountType {
    fn decode<D: Decoder>(d: &mut D) -> Result<AccountType, D::Error> {
        let s = try!(d.read_str());
        let default_acccount_type = AccountType::None;
        Ok(match &*s {
            "" => default_acccount_type,
            "current" => AccountType::Current,
            "savings" => AccountType::Savings,
            "business" => AccountType::Business,
            s => AccountType::Unknown(s.to_string()),
        })
    }
}

impl Decodable for Interval {
    fn decode<D: Decoder>(d: &mut D) -> Result<Interval, D::Error> {
        let s = try!(d.read_str());
        let default_interval = Interval::Monthly;
        Ok(match &*s {
            "" => default_interval,
            "monthly" => Interval::Monthly,
            _ => {
                error!("teller-cli currently only suports an interval of monthly");
                default_interval
            },
        })
    }
}

impl Decodable for Timeframe {
    fn decode<D: Decoder>(d: &mut D) -> Result<Timeframe, D::Error> {
        let s = try!(d.read_str());
        let default_timeframe = Timeframe::SixMonths;
        Ok(match &*s {
            "year" => Timeframe::Year,
            "6-months" => Timeframe::SixMonths,
            "3-months" => Timeframe::ThreeMonths,
            _ => default_timeframe,
        })
    }
}

impl Decodable for OutputFormat {
    fn decode<D: Decoder>(d: &mut D) -> Result<OutputFormat, D::Error> {
        let s = try!(d.read_str());
        let default_output_format = OutputFormat::Standard;
        Ok(match &*s {
            "spark" => OutputFormat::Spark,
            "standard" => OutputFormat::Standard,
            _ => default_output_format,
        })
    }
}

#[derive(Debug)]
pub enum CommandType {
    ShowUsage,
    Initialise,
    ListAccounts,
    ShowBalance,
    ShowOutgoing,
    ShowIncoming,
    ListTransactions,
    ListCounterparties,
    ListBalances,
    ListOutgoings,
    ListIncomings,
    None,
}

pub fn get_command_type(arguments: &CliArgs) -> CommandType {
    match *arguments {
        CliArgs { cmd_init, .. } if cmd_init == true => CommandType::Initialise,
        CliArgs { cmd_accounts, .. } if cmd_accounts == true => CommandType::ListAccounts,
        CliArgs { cmd_balance, .. } if cmd_balance == true => CommandType::ShowBalance,
        CliArgs { cmd_outgoing, .. } if cmd_outgoing == true => CommandType::ShowOutgoing,
        CliArgs { cmd_incoming, .. } if cmd_incoming == true => CommandType::ShowIncoming,
        CliArgs { cmd_transactions, .. } if cmd_transactions == true => CommandType::ListTransactions,
        CliArgs { cmd_counterparties, .. } if cmd_counterparties == true => CommandType::ListCounterparties,
        CliArgs { cmd_balances, .. } if cmd_balances == true => CommandType::ListBalances,
        CliArgs { cmd_incomings, .. } if cmd_incomings == true => CommandType::ListIncomings,
        CliArgs { cmd_outgoings, .. } if cmd_outgoings == true => CommandType::ListOutgoings,
        CliArgs { flag_help, flag_version, .. } if flag_help == true || flag_version == true => CommandType::None,
        _ => CommandType::ShowUsage,
    }
}
