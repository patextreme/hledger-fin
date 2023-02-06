use std::collections::{HashMap, HashSet};

use crate::{
    hledger::JournalEntry,
    input::Resource,
    model::{
        port::CashBalancePortfolio,
        txn::{cashbalance as cb, DatedTransaction},
        Commodity, PortId,
    },
};

struct CategorizedResources {
    portfolios: Vec<CashBalancePortfolio>,
    _commodities: HashSet<Commodity>,
    transactions: HashMap<PortId, Vec<cb::Transaction>>,
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
    let mut transactions: HashMap<PortId, Vec<cb::Transaction>> = HashMap::new();
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
                let tx = i.detail.into();
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
            Resource::Withdrawal(i) => {
                let tx = i.detail.into();
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
            Resource::Buy(i) => {
                let tx = i.detail.into();
                if let Some(txs) = transactions.get_mut(&i.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(i.port_id, vec![tx]);
                }
            }
            Resource::Sell(i) => {
                let tx = i.detail.into();
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

fn build_journal_from_transactions(transactions: Vec<cb::Transaction>) -> Vec<JournalEntry> {
    let mut sorted_transaction: Vec<(usize, cb::Transaction)> =
        transactions.into_iter().enumerate().collect();
    sorted_transaction.sort_by_key(|i| (i.1.date().clone(), i.0));

    for (_, txn) in sorted_transaction {
        todo!()
    }

    todo!()
}
