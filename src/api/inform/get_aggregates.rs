use cli::arg_types::Interval;

use std::str::FromStr; // Use of #from_str.

use itertools::Itertools;

use api::client::{TellerClient, ApiServiceResult, Transaction};
use api::client::parse_utc_date_from_transaction;
use chrono::{Date, Datelike, UTC};

pub type Balances = HistoricalAmountsWithCurrency;
pub type Outgoings = HistoricalAmountsWithCurrency;
pub type Incomings = HistoricalAmountsWithCurrency;

pub type IntervalAmount = (String, String);

type DateStringToTransactions = (String, Vec<Transaction>);

#[derive(Debug)]
pub struct HistoricalAmountsWithCurrency {
    pub historical_amounts: Vec<IntervalAmount>,
    pub currency: String,
}

impl HistoricalAmountsWithCurrency {
    pub fn new<S: Into<String>>(historical_amounts: Vec<IntervalAmount>,
                                currency: S)
                                -> HistoricalAmountsWithCurrency {
        HistoricalAmountsWithCurrency {
            historical_amounts: historical_amounts,
            currency: currency.into(),
        }
    }
}

pub trait GetBalances {
    fn get_balances(&self,
                    account_id: &str,
                    interval: &Interval,
                    from: &Date<UTC>,
                    to: &Date<UTC>) -> ApiServiceResult<Balances>;
}

pub trait GetOutgoings {
    fn get_outgoings(&self,
                     account_id: &str,
                     interval: &Interval,
                     from: &Date<UTC>,
                     to: &Date<UTC>) -> ApiServiceResult<Outgoings>;
}

pub trait GetIncomings {
    fn get_incomings(&self,
                     account_id: &str,
                     interval: &Interval,
                     from: &Date<UTC>,
                     to: &Date<UTC>) -> ApiServiceResult<Incomings>;
}

fn to_grouped_transaction_aggregates(transactions: Vec<Transaction>,
                                     from: &Date<UTC>,
                                     to: &Date<UTC>,
                                     interval: &Interval,
                                     aggregate_txs: &Fn(DateStringToTransactions) -> (String, i64))
                                     -> Vec<(String, i64)> {
    let group_format = |date: Date<UTC>| -> String {
        date.format("%m-%Y").to_string()
    };
    let get_txs_from_group = |current_date_str: &str, grouped_transactions: &Vec<DateStringToTransactions>| -> Vec<Transaction> {
        match grouped_transactions.iter().find(|g| g.0 == current_date_str) {
            Some(g) => g.1.clone(), // I had to make Transaction cloneable to do this...
            None => vec![],
        }
    };

    let month_year_grouped_transactions = transactions.into_iter().group_by(|t| {
        let transaction_date = parse_utc_date_from_transaction(&t);
        match *interval {
            Interval::Monthly => {
                let group_name = group_format(transaction_date);
                group_name
            }
        }
    }).collect();

    let mut month_year_transactions: Vec<DateStringToTransactions> = vec![];

    let mut current_date = from.clone();
    let end_date = to.clone();
    while current_date <= end_date {
        let current_date_str = group_format(current_date);

        let txs = get_txs_from_group(&current_date_str, &month_year_grouped_transactions);
        month_year_transactions.push((current_date_str, txs));

        let next_date = if current_date.month() < 12 {
            current_date.with_month(current_date.month() + 1).unwrap()
        } else {
            current_date.with_year(current_date.year() + 1).unwrap().with_month(1).unwrap()
        };
        current_date = next_date;
    }

    let mut month_year_aggregates = month_year_transactions.into_iter()
                                                           .map(aggregate_txs)
                                                           .collect::<Vec<(String, i64)>>();
    month_year_aggregates.reverse();

    month_year_aggregates
}

impl<'a> GetBalances for TellerClient<'a> {

    // The amount shown is for the beginning of a month before
    // any transactions have come in or out.
    //
    // NOTE: Balances will not work correctly if based off a different month
    //       than the current account balance returned by get_accounts.
    fn get_balances(&self,
                    account_id: &str,
                    interval: &Interval,
                    from: &Date<UTC>,
                    to: &Date<UTC>)
                    -> ApiServiceResult<Balances> {
        let sum_all = |myt: (String, Vec<Transaction>)| {
            let to_cent_integer = |t: &Transaction| {
                (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
            };

            let group_name = myt.0;
            let amount = myt.1.iter().map(to_cent_integer).fold(0i64, |sum, v| sum + v);
            (group_name, amount)
        };

        let account = try!(self.get_account(&account_id));
        let current_balance = (f64::from_str(&account.balance).unwrap() * 100f64).round() as i64;
        let currency = account.currency;

        let transactions = self.get_transactions(&account_id, &from, &to).unwrap_or(vec![]);
        let month_year_total_transactions = to_grouped_transaction_aggregates(transactions,
                                                                              &from,
                                                                              &to,
                                                                              &interval,
                                                                              &sum_all);

        let mut historical_amounts: Vec<IntervalAmount> = vec![];
        historical_amounts.push(("current".to_string(),
                                 format!("{:.2}", current_balance as f64 / 100f64)));

        let mut last_balance = current_balance;
        for mytt in month_year_total_transactions {
            last_balance = last_balance - mytt.1;
            historical_amounts.push((mytt.0.to_string(),
                                     format!("{:.2}", last_balance as f64 / 100f64)));
        }
        historical_amounts.reverse();

        Ok(HistoricalAmountsWithCurrency::new(historical_amounts, currency))
    }
}

impl<'a> GetOutgoings for TellerClient<'a> {
    fn get_outgoings(&self,
                     account_id: &str,
                     interval: &Interval,
                     from: &Date<UTC>,
                     to: &Date<UTC>)
                     -> ApiServiceResult<Outgoings> {
        let sum_outgoings = |myt: (String, Vec<Transaction>)| {
            let to_cent_integer = |t: &Transaction| {
                (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
            };

            let group_name = myt.0;
            let amount = myt.1
                            .iter()
                            .map(to_cent_integer)
                            .filter(|ci| *ci < 0)
                            .fold(0i64, |sum, v| sum + v);
            (group_name, amount)
        };

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        let transactions = self.get_transactions(&account_id, &from, &to).unwrap_or(vec![]);
        let month_year_total_outgoing = to_grouped_transaction_aggregates(transactions,
                                                                          &from,
                                                                          &to,
                                                                          &interval,
                                                                          &sum_outgoings);

        let from_cent_integer_to_float_string = |amount: i64| format!("{:.2}", amount as f64 / 100f64);

        let mut historical_amounts: Vec<IntervalAmount> = vec![];
        for mytt in month_year_total_outgoing {
            historical_amounts.push((mytt.0.to_string(),
                                     from_cent_integer_to_float_string(mytt.1.abs())));
        }
        historical_amounts.reverse();

        Ok(HistoricalAmountsWithCurrency::new(historical_amounts, currency))
    }
}

impl<'a> GetIncomings for TellerClient<'a> {
    fn get_incomings(&self,
                     account_id: &str,
                     interval: &Interval,
                     from: &Date<UTC>,
                     to: &Date<UTC>)
                     -> ApiServiceResult<Incomings> {
        let sum_incomings = |myt: (String, Vec<Transaction>)| {
            let to_cent_integer = |t: &Transaction| {
                (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
            };

            let group_name = myt.0;
            let amount = myt.1
                            .iter()
                            .map(to_cent_integer)
                            .filter(|ci| *ci > 0)
                            .fold(0i64, |sum, v| sum + v);
            (group_name, amount)
        };

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        let transactions = self.get_transactions(&account_id, &from, &to).unwrap_or(vec![]);
        let month_year_total_incoming = to_grouped_transaction_aggregates(transactions,
                                                                          &from,
                                                                          &to,
                                                                          &interval,
                                                                          &sum_incomings);

        let from_cent_integer_to_float_string = |amount: i64| format!("{:.2}", amount as f64 / 100f64);

        let mut historical_amounts: Vec<IntervalAmount> = vec![];
        for mytt in month_year_total_incoming {
            historical_amounts.push((mytt.0.to_string(),
                                     from_cent_integer_to_float_string(mytt.1)));
        }
        historical_amounts.reverse();

        Ok(HistoricalAmountsWithCurrency::new(historical_amounts, currency))
    }
}

#[cfg(test)]
mod tests {
    use cli::arg_types::Interval;

    use api::client::{TellerClient, generate_utc_date_from_date_str};
    use super::{GetBalances, GetOutgoings, GetIncomings};

    use hyper;
    mock_connector_in_order!(GetAccountFollowedByGetTransactions {
        include_str!("../mocks/get-account.http")
        include_str!("../mocks/get-transactions.http")
    });

    #[test]
    fn can_get_balances() {
        let c = hyper::client::Client::with_connector(GetAccountFollowedByGetTransactions::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let from = generate_utc_date_from_date_str("2015-01-01");
        let to = generate_utc_date_from_date_str("2015-12-31");
        let agg = teller.get_balances("123", &Interval::Monthly, &from, &to).unwrap();

        assert_eq!("GBP", agg.currency);
        assert_eq!("01-2015", agg.historical_amounts[0].0);
        assert_eq!("858.97", agg.historical_amounts[0].1);
        assert_eq!("02-2015", agg.historical_amounts[1].0);
        assert_eq!("835.00", agg.historical_amounts[1].1);
        assert_eq!("03-2015", agg.historical_amounts[2].0);
        assert_eq!("835.00", agg.historical_amounts[2].1);
        assert_eq!("04-2015", agg.historical_amounts[3].0);
        assert_eq!("835.00", agg.historical_amounts[3].1);
        assert_eq!("05-2015", agg.historical_amounts[4].0);
        assert_eq!("835.00", agg.historical_amounts[4].1);
        assert_eq!("06-2015", agg.historical_amounts[5].0);
        assert_eq!("810.00", agg.historical_amounts[5].1);
        assert_eq!("07-2015", agg.historical_amounts[6].0);
        assert_eq!("760.00", agg.historical_amounts[6].1);
        assert_eq!("08-2015", agg.historical_amounts[7].0);
        assert_eq!("910.00", agg.historical_amounts[7].1);
        assert_eq!("09-2015", agg.historical_amounts[8].0);
        assert_eq!("1010.00", agg.historical_amounts[8].1);
        assert_eq!("10-2015", agg.historical_amounts[9].0);
        assert_eq!("960.00", agg.historical_amounts[9].1);
        assert_eq!("11-2015", agg.historical_amounts[10].0);
        assert_eq!("1010.00", agg.historical_amounts[10].1);
        assert_eq!("12-2015", agg.historical_amounts[11].0);
        assert_eq!("950.00", agg.historical_amounts[11].1);
        assert_eq!("current", agg.historical_amounts[12].0);
        assert_eq!("1000.00", agg.historical_amounts[12].1);
    }

    #[test]
    fn can_get_outgoings() {
        let c = hyper::client::Client::with_connector(GetAccountFollowedByGetTransactions::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let from = generate_utc_date_from_date_str("2015-01-01");
        let to = generate_utc_date_from_date_str("2015-12-31");
        let agg = teller.get_outgoings("123", &Interval::Monthly, &from, &to).unwrap();

        assert_eq!("GBP", agg.currency);
        assert_eq!("01-2015", agg.historical_amounts[0].0);
        assert_eq!("23.97", agg.historical_amounts[0].1);
        assert_eq!("02-2015", agg.historical_amounts[1].0);
        assert_eq!("0.00", agg.historical_amounts[1].1);
        assert_eq!("03-2015", agg.historical_amounts[2].0);
        assert_eq!("0.00", agg.historical_amounts[2].1);
        assert_eq!("04-2015", agg.historical_amounts[3].0);
        assert_eq!("0.00", agg.historical_amounts[3].1);
        assert_eq!("05-2015", agg.historical_amounts[4].0);
        assert_eq!("25.00", agg.historical_amounts[4].1);
        assert_eq!("06-2015", agg.historical_amounts[5].0);
        assert_eq!("50.00", agg.historical_amounts[5].1);
        assert_eq!("07-2015", agg.historical_amounts[6].0);
        assert_eq!("0.00", agg.historical_amounts[6].1);
        assert_eq!("08-2015", agg.historical_amounts[7].0);
        assert_eq!("0.00", agg.historical_amounts[7].1);
        assert_eq!("09-2015", agg.historical_amounts[8].0);
        assert_eq!("50.00", agg.historical_amounts[8].1);
        assert_eq!("10-2015", agg.historical_amounts[9].0);
        assert_eq!("0.00", agg.historical_amounts[9].1);
        assert_eq!("11-2015", agg.historical_amounts[10].0);
        assert_eq!("60.00", agg.historical_amounts[10].1);
        assert_eq!("12-2015", agg.historical_amounts[11].0);
        assert_eq!("0.00", agg.historical_amounts[11].1);
    }

    #[test]
    fn can_get_incomings() {
        let c = hyper::client::Client::with_connector(GetAccountFollowedByGetTransactions::default());
        let teller = TellerClient::new_with_hyper_client("fake-auth-token", c);

        let from = generate_utc_date_from_date_str("2015-01-01");
        let to = generate_utc_date_from_date_str("2015-12-31");
        let agg = teller.get_incomings("123", &Interval::Monthly, &from, &to).unwrap();

        assert_eq!("GBP", agg.currency);
        assert_eq!("01-2015", agg.historical_amounts[0].0);
        assert_eq!("0.00", agg.historical_amounts[0].1);
        assert_eq!("02-2015", agg.historical_amounts[1].0);
        assert_eq!("0.00", agg.historical_amounts[1].1);
        assert_eq!("03-2015", agg.historical_amounts[2].0);
        assert_eq!("0.00", agg.historical_amounts[2].1);
        assert_eq!("04-2015", agg.historical_amounts[3].0);
        assert_eq!("0.00", agg.historical_amounts[3].1);
        assert_eq!("05-2015", agg.historical_amounts[4].0);
        assert_eq!("0.00", agg.historical_amounts[4].1);
        assert_eq!("06-2015", agg.historical_amounts[5].0);
        assert_eq!("0.00", agg.historical_amounts[5].1);
        assert_eq!("07-2015", agg.historical_amounts[6].0);
        assert_eq!("150.00", agg.historical_amounts[6].1);
        assert_eq!("08-2015", agg.historical_amounts[7].0);
        assert_eq!("100.00", agg.historical_amounts[7].1);
        assert_eq!("09-2015", agg.historical_amounts[8].0);
        assert_eq!("0.00", agg.historical_amounts[8].1);
        assert_eq!("10-2015", agg.historical_amounts[9].0);
        assert_eq!("50.00", agg.historical_amounts[9].1);
        assert_eq!("11-2015", agg.historical_amounts[10].0);
        assert_eq!("0.00", agg.historical_amounts[10].1);
        assert_eq!("12-2015", agg.historical_amounts[11].0);
        assert_eq!("50.00", agg.historical_amounts[11].1);
    }

}
