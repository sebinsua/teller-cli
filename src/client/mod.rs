pub mod error;

use cli::arg_types::{Interval, Timeframe};

use hyper::{Client, Url};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;
use chrono::{Date, DateTime, UTC, Datelike};
use chrono::duration::Duration;
use itertools::Itertools;

use std::collections::HashMap;

use std::io::prelude::*; // Required for read_to_string use later.
use std::str::FromStr;

use self::error::TellerClientError;

pub type ApiServiceResult<T> = Result<T, TellerClientError>;

pub type IntervalAmount = (String, String);
pub type Balances = HistoricalAmountsWithCurrency;
pub type Outgoings = HistoricalAmountsWithCurrency;
pub type Incomings = HistoricalAmountsWithCurrency;
type DateStringToTransactions = (String, Vec<Transaction>);

#[derive(Debug, RustcDecodable)]
struct AccountResponse {
    data: Account,
}

#[derive(Debug, RustcDecodable)]
struct AccountsResponse {
    data: Vec<Account>,
}

#[derive(Debug, RustcDecodable)]
struct TransactionsResponse {
    data: Vec<Transaction>,
}

#[derive(Debug, RustcDecodable)]
pub struct Account {
    pub updated_at: String,
    pub institution: String,
    pub id: String,
    pub currency: String,
    pub balance: String,
    pub account_number_last_4: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Transaction {
    pub description: String,
    pub date: String,
    pub counterparty: String,
    pub amount: String,
}

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

fn get_auth_header(auth_token: &str) -> Authorization<Bearer> {
    Authorization(Bearer { token: auth_token.to_string() })
}

fn parse_utc_date_from_transaction(t: &Transaction) -> Date<UTC> {
    let full_date = &(t.date.to_owned() + "T00:00:00-00:00");
    let past_transaction_date_without_tz = DateTime::parse_from_rfc3339(full_date).unwrap().date();
    let past_transaction_date = past_transaction_date_without_tz.with_timezone(&UTC);
    past_transaction_date
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

const TELLER_API_SERVER_URL: &'static str = "https://api.teller.io";

pub struct TellerClient<'a> {
    auth_token: &'a str,
}

impl<'a> TellerClient<'a> {
    pub fn new(auth_token: &'a str) -> TellerClient {
        TellerClient {
            auth_token: auth_token,
        }
    }

    fn get_body(&self, url: &str) -> ApiServiceResult<String> {
        let client = Client::new();
        let mut res = try!(client.get(url)
                                 .header(get_auth_header(&self.auth_token))
                                 .send());
        if res.status.is_client_error() {
            return Err(TellerClientError::AuthenticationError);
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        debug!("GET {} response: {}", url, body);

        Ok(body)
    }

    pub fn get_accounts(&self) -> ApiServiceResult<Vec<Account>> {
        let body = try!(self.get_body(&format!("{}/accounts", TELLER_API_SERVER_URL)));
        let accounts_response: AccountsResponse = try!(json::decode(&body));

        Ok(accounts_response.data)
    }

    pub fn get_account(&self, account_id: &str) -> ApiServiceResult<Account> {
        let body = try!(self.get_body(&format!("{}/accounts/{}", TELLER_API_SERVER_URL, account_id)));
        let account_response: AccountResponse = try!(json::decode(&body));

        Ok(account_response.data)
    }

    // TODO: INFORM: Move elsewhere.
    pub fn get_account_balance(&self, account_id: &str) -> ApiServiceResult<Money> {
        let to_money = |a: Account| Money::new(a.balance, a.currency);
        self.get_account(&account_id).map(to_money)
    }

    pub fn raw_transactions(&self,
                            account_id: &str,
                            count: u32,
                            page: u32)
                            -> ApiServiceResult<Vec<Transaction>> {
        let mut url = Url::parse(&format!("{}/accounts/{}/transactions",
                                          TELLER_API_SERVER_URL,
                                          account_id)).unwrap();

        const COUNT: &'static str = "count";
        const PAGE: &'static str = "page";
        let query = vec![(COUNT, count.to_string()), (PAGE, page.to_string())];
        url.set_query_from_pairs(query.into_iter());

        let body = try!(self.get_body(&url.serialize()));
        let transactions_response: TransactionsResponse = try!(json::decode(&body));

        Ok(transactions_response.data)
    }

    pub fn get_transactions(&self,
                            account_id: &str,
                            timeframe: &Timeframe)
                            -> ApiServiceResult<Vec<Transaction>> {
        let page_through_transactions = |from| -> ApiServiceResult<Vec<Transaction>> {
            let mut all_transactions = vec![];

            let mut fetching = true;
            let mut page = 1;
            let count = 250;
            while fetching {
                let mut transactions = try!(self.raw_transactions(&account_id, count, page));
                match transactions.last() {
                    None => {
                        // If there are no transactions left, do not fetch forever...
                        fetching = false
                    }
                    Some(past_transaction) => {
                        let past_transaction_date = parse_utc_date_from_transaction(&past_transaction);
                        if past_transaction_date < from {
                            fetching = false;
                        }
                    }
                };

                all_transactions.append(&mut transactions);
                page = page + 1;
            }

            all_transactions = all_transactions.into_iter()
                                               .filter(|t| {
                                                   let transaction_date =
                                                       parse_utc_date_from_transaction(&t);
                                                   transaction_date > from
                                               })
                                               .collect();

            all_transactions.reverse();
            Ok(all_transactions)
        };

        match *timeframe {
            Timeframe::ThreeMonths => {
                let to = UTC::today();
                let from = to - Duration::days(91); // close enough... 😅

                page_through_transactions(from)
            }
            Timeframe::SixMonths => {
                let to = UTC::today();
                let from = to - Duration::days(183);

                page_through_transactions(from)
            }
            Timeframe::Year => {
                let to = UTC::today();
                let from = to - Duration::days(365);

                page_through_transactions(from)
            }
        }
    }

    pub fn get_transactions_with_currency(&self,
                                          account_id: &str,
                                          timeframe: &Timeframe)
                                          -> ApiServiceResult<TransactionsWithCurrrency> {
        let transactions = try!(self.get_transactions(&account_id, &timeframe));

        let account = try!(self.get_account(&account_id));
        let currency = account.currency;

        Ok(TransactionsWithCurrrency::new(transactions, currency))
    }

    pub fn get_counterparties(&self,
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

    fn get_grouped_transaction_aggregates(&self,
                                          account_id: &str,
                                          interval: &Interval,
                                          timeframe: &Timeframe,
                                          aggregate_txs: &Fn(DateStringToTransactions) -> (String, i64))
                                          -> ApiServiceResult<Vec<(String, i64)>> {
        let transactions: Vec<Transaction> = self.get_transactions(&account_id, &timeframe)
                                                 .unwrap_or(vec![]);

        let mut month_year_grouped_transactions: Vec<(String, i64)> =
            transactions.into_iter()
                        .group_by(|t| {
                            let transaction_date = parse_utc_date_from_transaction(&t);
                            match *interval {
                                Interval::Monthly => {
                                    let group_name = transaction_date.format("%m-%Y").to_string();
                                    group_name
                                }
                            }
                        })
                        .map(aggregate_txs)
                        .collect();
        month_year_grouped_transactions.reverse();

        Ok(month_year_grouped_transactions)
    }

    pub fn get_balances(&self,
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

        let month_year_total_transactions = try!(self.get_grouped_transaction_aggregates(&account_id,
                                                                                    &interval,
                                                                                    &timeframe,
                                                                                    &sum_all));

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

    pub fn get_outgoings(&self,
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

        let month_year_total_outgoing = try!(self.get_grouped_transaction_aggregates(&account_id,
                                                                                &interval,
                                                                                &timeframe,
                                                                                &sum_outgoings));

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

    pub fn get_incomings(&self,
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

        let month_year_total_incoming = try!(self.get_grouped_transaction_aggregates(&account_id,
                                                                                &interval,
                                                                                &timeframe,
                                                                                &sum_incomings));

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

    pub fn get_outgoing(&self, account_id: &str) -> ApiServiceResult<Money> {
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

    pub fn get_incoming(&self, account_id: &str) -> ApiServiceResult<Money> {
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
