use cli::arg_types::{Interval, Timeframe};

use chrono::{UTC, Datelike};
use itertools::Itertools;

use std::collections::HashMap;

use std::str::FromStr; // Use of #from_str.

use super::client::{TellerClient, ApiServiceResult, Transaction, Account};
use super::client::parse_utc_date_from_transaction;

pub type IntervalAmount = (String, String);
pub type Balances = HistoricalAmountsWithCurrency;
pub type Outgoings = HistoricalAmountsWithCurrency;
pub type Incomings = HistoricalAmountsWithCurrency;
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

#[derive(Debug)]
pub struct TransactionsWithCurrrency {
    pub transactions: Vec<Transaction>,
    pub currency: String,
}

impl TransactionsWithCurrrency {
    pub fn new<S: Into<String>>(transactions: Vec<Transaction>,
                                currency: S)
                                -> TransactionsWithCurrrency {
        TransactionsWithCurrrency {
            transactions: transactions,
            currency: currency.into(),
        }
    }
}

#[derive(Debug)]
pub struct CounterpartiesWithCurrrency {
    pub counterparties: Vec<(String, String)>,
    pub currency: String,
}

impl CounterpartiesWithCurrrency {
    pub fn new<S: Into<String>>(counterparties: Vec<(String, String)>,
                                currency: S)
                                -> CounterpartiesWithCurrrency {
        CounterpartiesWithCurrrency {
            counterparties: counterparties,
            currency: currency.into(),
        }
    }
}

#[derive(Debug)]
pub struct Money {
    amount: String,
    currency: String,
}

impl Money {
    pub fn new<S: Into<String>>(amount: S, currency: S) -> Money {
        Money {
            amount: amount.into(),
            currency: currency.into(),
        }
    }

    pub fn get_balance_for_display(&self, hide_currency: &bool) -> String {
        if *hide_currency {
            self.amount.to_owned()
        } else {
            let balance_with_currency = format!("{} {}", self.amount, self.currency);
            balance_with_currency.to_owned()
        }
    }
}

pub trait GetAccountBalance {
    fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money>;
}

impl<'a> GetAccountBalance for TellerClient<'a> {
    fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money> {
        let to_money = |a: Account| Money::new(a.balance, a.currency);
        self.get_account(&account_id).map(to_money)
    }
}

pub trait GetIncoming {
    fn get_incoming(&self, account_id: &str) -> ApiServiceResult<Money>;
}

impl<'a> GetIncoming for TellerClient<'a> {
    fn get_incoming(&self, account_id: &str) -> ApiServiceResult<Money> {
        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        let from = UTC::today().with_day(1).unwrap();
        let transactions: Vec<Transaction> = self.raw_transactions(&account_id, 250, 1)
                                                 .unwrap_or(vec![])
                                                 .into_iter()
                                                 .filter(|t| {
                                                     let transaction_date =
                                                         parse_utc_date_from_transaction(&t);
                                                     transaction_date > from
                                                 })
                                                 .collect();

        let from_float_string_to_cent_integer = |t: &Transaction| {
            (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
        };
        let from_cent_integer_to_float_string = |amount: i64| format!("{:.2}", amount as f64 / 100f64);

        let incoming = transactions.iter()
                                   .map(from_float_string_to_cent_integer)
                                   .filter(|ci| *ci > 0)
                                   .fold(0i64, |sum, v| sum + v);

        Ok(Money::new(from_cent_integer_to_float_string(incoming), currency))
    }
}

pub trait GetOutgoing {
    fn get_outgoing(&self, account_id: &str) -> ApiServiceResult<Money>;
}

impl<'a> GetOutgoing for TellerClient<'a> {
    fn get_outgoing(&self, account_id: &str) -> ApiServiceResult<Money> {
        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        let from = UTC::today().with_day(1).unwrap();
        let transactions: Vec<Transaction> = self.raw_transactions(&account_id, 250, 1)
                                                 .unwrap_or(vec![])
                                                 .into_iter()
                                                 .filter(|t| {
                                                     let transaction_date =
                                                         parse_utc_date_from_transaction(&t);
                                                     transaction_date > from
                                                 })
                                                 .collect();

        let from_float_string_to_cent_integer = |t: &Transaction| {
            (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
        };
        let from_cent_integer_to_float_string = |amount: i64| format!("{:.2}", amount as f64 / 100f64);

        let outgoing = transactions.iter()
                                   .map(from_float_string_to_cent_integer)
                                   .filter(|ci| *ci < 0)
                                   .fold(0i64, |sum, v| sum + v);

        Ok(Money::new(from_cent_integer_to_float_string(outgoing.abs()), currency))
    }
}

pub trait GetTransactionsWithCurrency {
    fn get_transactions_with_currency(&self,
                                          account_id: &str,
                                          timeframe: &Timeframe)
                                          -> ApiServiceResult<TransactionsWithCurrrency>;
}

impl<'a> GetTransactionsWithCurrency for TellerClient<'a> {
    fn get_transactions_with_currency(&self,
                                          account_id: &str,
                                          timeframe: &Timeframe)
                                          -> ApiServiceResult<TransactionsWithCurrrency> {
        let transactions = try!(self.get_transactions(&account_id, &timeframe));

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        Ok(TransactionsWithCurrrency::new(transactions, currency))
    }
}

pub trait GetCounterparties {
    fn get_counterparties(&self,
                          account_id: &str,
                          timeframe: &Timeframe) -> ApiServiceResult<CounterpartiesWithCurrrency>;
}

fn convert_to_counterparty_to_date_amount_list<'a>(transactions: &'a Vec<Transaction>)
                                                   -> HashMap<String, Vec<(String, String)>> {
    let grouped_counterparties = transactions.iter()
                                             .fold(HashMap::new(), |mut acc: HashMap<String,
                                                                     Vec<&'a Transaction>>,
                                                    t: &'a Transaction| {
                                                 let counterparty = t.counterparty.to_owned();
                                                 if acc.contains_key(&counterparty) {
                                                     if let Some(txs) = acc.get_mut(&counterparty) {
                                                         txs.push(t);
                                                     }
                                                 } else {
                                                     let mut txs: Vec<&'a Transaction> = vec![];
                                                     txs.push(t);
                                                     acc.insert(counterparty, txs);
                                                 }

                                                 acc
                                             });

    grouped_counterparties.into_iter().fold(HashMap::new(), |mut acc, (counterparty, txs)| {
        let date_amount_tuples = txs.into_iter()
                                    .map(|tx| (tx.date.to_owned(), tx.amount.to_owned()))
                                    .collect();
        acc.insert(counterparty.to_string(), date_amount_tuples);
        acc
    })
}

impl<'a> GetCounterparties for TellerClient<'a> {
    fn get_counterparties(&self,
                              account_id: &str,
                              timeframe: &Timeframe)
                              -> ApiServiceResult<CounterpartiesWithCurrrency> {
        let transactions_with_currency = try!(self.get_transactions_with_currency(&account_id,
                                                                                  &timeframe));

        let to_cent_integer = |amount: &str| (f64::from_str(&amount).unwrap() * 100f64).round() as i64;
        let from_cent_integer_to_float_string = |amount: &i64| {
            format!("{:.2}", *amount as f64 / 100f64)
        };

        let transactions: Vec<Transaction> = transactions_with_currency.transactions
                                                                       .into_iter()
                                                                       .filter(|tx| {
                                                                           to_cent_integer(&tx.amount) <
                                                                           0
                                                                       })
                                                                       .collect();
        let currency = transactions_with_currency.currency;

        let counterparty_to_date_amount_list =
            convert_to_counterparty_to_date_amount_list(&transactions);
        let sorted_counterparties =
            counterparty_to_date_amount_list.into_iter()
                                            .map(|(counterparty, date_amount_tuples)| {
                                                let amount =
                                                    date_amount_tuples.iter().fold(0i64, |acc, dat| {
                                                        acc + to_cent_integer(&dat.1)
                                                    });
                                                (counterparty, amount.abs())
                                            })
                                            .sort_by(|&(_, amount_a), &(_, amount_b)| {
                                                amount_a.cmp(&amount_b)
                                            });
        let counterparties = sorted_counterparties.into_iter()
                                                  .map(|(counterparty, amount)| {
                                                      (counterparty,
                                                       from_cent_integer_to_float_string(&amount))
                                                  })
                                                  .collect();

        Ok(CounterpartiesWithCurrrency::new(counterparties, currency))
    }
}

pub trait GetBalances {
    fn get_balances(&self,
                    account_id: &str,
                    interval: &Interval,
                    timeframe: &Timeframe) -> ApiServiceResult<Balances>;
}

pub trait GetOutgoings {
    fn get_outgoings(&self,
                     account_id: &str,
                     interval: &Interval,
                     timeframe: &Timeframe) -> ApiServiceResult<Outgoings>;
}

pub trait GetIncomings {
    fn get_incomings(&self,
                     account_id: &str,
                     interval: &Interval,
                     timeframe: &Timeframe) -> ApiServiceResult<Incomings>;
}

fn to_grouped_transaction_aggregates(transactions: Vec<Transaction>,
                                     interval: &Interval,
                                     aggregate_txs: &Fn(DateStringToTransactions) -> (String, i64))
                                     -> Vec<(String, i64)> {
    let mut month_year_grouped_transactions: Vec<(String, i64)> = transactions.into_iter().group_by(|t| {
        let transaction_date = parse_utc_date_from_transaction(&t);
        match *interval {
            Interval::Monthly => {
                let group_name = transaction_date.format("%m-%Y").to_string();
                group_name
            }
        }
    }).map(aggregate_txs).collect();
    month_year_grouped_transactions.reverse();

    month_year_grouped_transactions
}

impl<'a> GetBalances for TellerClient<'a> {
    fn get_balances(&self,
                    account_id: &str,
                    interval: &Interval,
                    timeframe: &Timeframe)
                    -> ApiServiceResult<Balances> {
        let sum_all = |myt: (String, Vec<Transaction>)| {
            let to_cent_integer = |t: &Transaction| {
                (f64::from_str(&t.amount).unwrap() * 100f64).round() as i64
            };

            let group_name = myt.0;
            let amount = myt.1.iter().map(to_cent_integer).fold(0i64, |sum, v| sum + v);
            (group_name, amount)
        };

        let transactions = self.get_transactions(&account_id, &timeframe).unwrap_or(vec![]);
        let month_year_total_transactions = to_grouped_transaction_aggregates(transactions,
                                                                              &interval,
                                                                              &sum_all);

        let account = try!(self.get_account(&account_id));
        let current_balance = (f64::from_str(&account.balance).unwrap() * 100f64).round() as i64;
        let currency = account.currency;

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
                     timeframe: &Timeframe)
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

        let transactions = self.get_transactions(&account_id, &timeframe).unwrap_or(vec![]);
        let month_year_total_outgoing = to_grouped_transaction_aggregates(transactions,
                                                                          &interval,
                                                                          &sum_outgoings);

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

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
                     timeframe: &Timeframe)
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

        let transactions = self.get_transactions(&account_id, &timeframe).unwrap_or(vec![]);
        let month_year_total_incoming = to_grouped_transaction_aggregates(transactions,
                                                                          &interval,
                                                                          &sum_incomings);

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

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
