use std::collections::{HashMap, HashSet};

use crate::{
    hledger::JournalEntry,
    input::Resource,
    inventory::FifoInventory,
    model::{
        port::CashBalancePortfolio,
        txn::{Buy, DatedTransaction, Deposit, Sell, Withdraw},
        Commodity, PortId,
    },
};

struct CategorizedResources {
    portfolios: Vec<CashBalancePortfolio>,
    _commodities: HashSet<Commodity>, // TODO: validate commodity
    transactions: HashMap<PortId, Vec<Box<dyn BookkeepingAndDated<CashBalancePortfolio>>>>,
}

pub fn build_journal(resources: Vec<Resource>) -> Vec<JournalEntry> {
    let mut categorized_resources = categorize_resources(resources);
    let mut result = Vec::new();
    for port in categorized_resources.portfolios {
        if let Some(transactions) = categorized_resources.transactions.remove(&port.port_id) {
            let entries = build_journal_from_transactions(transactions);
            result.extend(entries);
        }
    }
    result
}

fn categorize_resources(resources: Vec<Resource>) -> CategorizedResources {
    let mut portfolios: Vec<CashBalancePortfolio> = Vec::new();
    let mut commodities: HashSet<Commodity> = HashSet::new();
    let mut transactions: HashMap<PortId, Vec<Box<dyn BookkeepingAndDated<CashBalancePortfolio>>>> =
        HashMap::new();
    for r in resources {
        match r {
            Resource::CashBalancePortfolio(port) => {
                portfolios.push(*port);
            }
            Resource::Commodity(c) => {
                commodities.insert(c);
            }
            Resource::CommodityList(ls) => {
                for c in ls {
                    commodities.insert(c);
                }
            }
            Resource::Deposit(i) => {
                let tx = Box::new(i.detail);
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
            Resource::Withdrawal(i) => {
                let tx = Box::new(i.detail);
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
            Resource::Buy(i) => {
                let tx = Box::new(i.detail);
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
            Resource::Sell(i) => {
                let tx = Box::new(i.detail);
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
        }
    }

    CategorizedResources {
        portfolios,
        _commodities: commodities,
        transactions,
    }
}

fn build_journal_from_transactions(
    transactions: Vec<Box<dyn BookkeepingAndDated<CashBalancePortfolio>>>,
) -> Vec<JournalEntry> {
    let mut sorted_transaction: Vec<(usize, Box<dyn BookkeepingAndDated<CashBalancePortfolio>>)> =
        transactions.into_iter().enumerate().collect();
    sorted_transaction.sort_by_key(|i| (i.1.date().clone(), i.0));

    todo!()
}

trait Bookkeeping<Port: 'static> {
    fn to_entries(&self, port: &Port) -> Vec<JournalEntry>;
}

impl Bookkeeping<CashBalancePortfolio> for Deposit {
    fn to_entries(&self, port: &CashBalancePortfolio) -> Vec<JournalEntry> {
        todo!()
    }
}

impl Bookkeeping<CashBalancePortfolio> for Withdraw {
    fn to_entries(&self, port: &CashBalancePortfolio) -> Vec<JournalEntry> {
        todo!()
    }
}

impl Bookkeeping<CashBalancePortfolio> for Buy {
    fn to_entries(&self, port: &CashBalancePortfolio) -> Vec<JournalEntry> {
        todo!()
    }
}

impl Bookkeeping<CashBalancePortfolio> for Sell {
    fn to_entries(&self, port: &CashBalancePortfolio) -> Vec<JournalEntry> {
        todo!()
    }
}

trait BookkeepingAndDated<Port: 'static>:
    Bookkeeping<Port> + DatedTransaction + std::fmt::Debug
{
}

impl<Port: 'static, T: DatedTransaction + Bookkeeping<Port> + std::fmt::Debug>
    BookkeepingAndDated<Port> for T
{
}
