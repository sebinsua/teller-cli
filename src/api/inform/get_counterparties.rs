use cli::arg_types::Timeframe;

use std::str::FromStr; // Use of #from_str.

use std::collections::HashMap;
use itertools::Itertools;

use api::client::{TellerClient, ApiServiceResult, Transaction};

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

pub trait GetCounterparties {
    fn get_counterparties(&self,
                          account_id: &str,
                          timeframe: &Timeframe) -> ApiServiceResult<CounterpartiesWithCurrrency>;
}

fn convert_to_counterparty_to_date_amount_list<'a>(transactions: &'a Vec<Transaction>)
                                                   -> HashMap<String, Vec<(String, String)>> {
    let grouped_counterparties = transactions
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<String, Vec<&'a Transaction>>, t: &'a Transaction| {
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
        let transactions = try!(self.get_transactions(&account_id, &timeframe));
        let account = try!(self.get_account(&account_id));

        let to_cent_integer = |amount: &str| (f64::from_str(&amount).unwrap() * 100f64).round() as i64;
        let from_cent_integer_to_float_string = |amount: &i64| {
            format!("{:.2}", *amount as f64 / 100f64)
        };

        let outgoing_transactions: Vec<Transaction> = transactions.into_iter().filter(|tx| {
            to_cent_integer(&tx.amount) < 0
        }).collect();

        let counterparty_to_date_amount_list =
            convert_to_counterparty_to_date_amount_list(&outgoing_transactions);
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
        let currency = account.currency;
        Ok(CounterpartiesWithCurrrency::new(counterparties, currency))
    }
}
