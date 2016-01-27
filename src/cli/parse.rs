use rustc_serialize::{Decodable, Decoder};

use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

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
            }
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

#[derive(Debug, PartialEq)]
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
        CliArgs { cmd_init, .. } if cmd_init => CommandType::Initialise,
        CliArgs { cmd_accounts, .. } if cmd_accounts => CommandType::ListAccounts,
        CliArgs { cmd_balance, .. } if cmd_balance => CommandType::ShowBalance,
        CliArgs { cmd_outgoing, .. } if cmd_outgoing => CommandType::ShowOutgoing,
        CliArgs { cmd_incoming, .. } if cmd_incoming => CommandType::ShowIncoming,
        CliArgs { cmd_transactions, .. } if cmd_transactions => CommandType::ListTransactions,
        CliArgs { cmd_counterparties, .. } if cmd_counterparties => CommandType::ListCounterparties,
        CliArgs { cmd_balances, .. } if cmd_balances => CommandType::ListBalances,
        CliArgs { cmd_incomings, .. } if cmd_incomings => CommandType::ListIncomings,
        CliArgs { cmd_outgoings, .. } if cmd_outgoings => CommandType::ListOutgoings,
        CliArgs { flag_help, flag_version, .. } if flag_help || flag_version => CommandType::None,
        _ => CommandType::ShowUsage,
    }
}

#[cfg(test)]
mod tests {
    use super::CliArgs;
    use super::CommandType;
    use super::get_command_type;

    use cli::arg_types::{AccountType, OutputFormat, Interval, Timeframe};

    #[test]
    fn can_fallback_to_show_usage_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: false,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ShowUsage, command_type);
    }

    #[test]
    fn can_get_init_command_type() {
        let args = CliArgs {
            cmd_init: true,
            cmd_list: false,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::Initialise, command_type);
    }

    #[test]
    fn can_get_list_accounts_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: true,
            cmd_show: false,
            cmd_accounts: true,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ListAccounts, command_type);
    }

    #[test]
    fn can_get_list_transactions_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: true,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: true,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ListTransactions, command_type);
    }

    #[test]
    fn can_get_list_counterparties_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: true,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: true,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ListCounterparties, command_type);
    }

    #[test]
    fn can_get_list_balances_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: true,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: true,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ListBalances, command_type);
    }

    #[test]
    fn can_get_list_outgoings_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: true,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: true,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ListOutgoings, command_type);
    }

    #[test]
    fn can_get_list_incomings_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: true,
            cmd_show: false,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: true,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ListIncomings, command_type);
    }

    #[test]
    fn can_get_show_balance_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: false,
            cmd_show: true,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: true,
            cmd_outgoing: false,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ShowBalance, command_type);
    }

    #[test]
    fn can_get_show_outgoing_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: false,
            cmd_show: true,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: true,
            cmd_incoming: false,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ShowOutgoing, command_type);
    }

    #[test]
    fn can_get_show_incoming_command_type() {
        let args = CliArgs {
            cmd_init: false,
            cmd_list: false,
            cmd_show: true,
            cmd_accounts: false,
            cmd_transactions: false,
            cmd_counterparties: false,
            cmd_balances: false,
            cmd_outgoings: false,
            cmd_incomings: false,
            cmd_balance: false,
            cmd_outgoing: false,
            cmd_incoming: true,
            arg_account: AccountType::None,
            flag_interval: Interval::Monthly,
            flag_timeframe: Timeframe::Year,
            flag_count: 0i64,
            flag_show_description: false,
            flag_hide_currency: false,
            flag_output: OutputFormat::Standard,
            flag_help: false,
            flag_version: false,
        };

        let command_type = get_command_type(&args);

        assert_eq!(CommandType::ShowIncoming, command_type);
    }

}
