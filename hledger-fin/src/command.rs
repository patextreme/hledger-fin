use std::collections::{HashMap, HashSet};

use crate::{
    hledger::JournalEntry,
    input::Resource,
    model::{port::CashBalancePortfolio, txn::CashBalanceTransaction, Commodity, PortId},
};

#[derive(Debug, Clone)]
struct CategorizedResources {
    portfolios: HashMap<PortId, CashBalancePortfolio>,
    commodities: HashSet<Commodity>,
    transactions: HashMap<PortId, Vec<CashBalanceTransaction>>,
}

pub fn build_journal(resources: Vec<Resource>) -> Vec<JournalEntry> {
    let categorized_resources = categorize_resources(resources);

    println!("{categorized_resources:#?}");

    todo!()
}

fn categorize_resources(resources: Vec<Resource>) -> CategorizedResources {
    let mut portfolios: HashMap<PortId, CashBalancePortfolio> = HashMap::new();
    let mut commodities: HashSet<Commodity> = HashSet::new();
    let mut transactions: HashMap<PortId, Vec<CashBalanceTransaction>> = HashMap::new();
    for r in resources {
        match r {
            Resource::CashBalancePortfolio(port) => {
                portfolios.insert(port.port_id.clone(), *port);
            }
            Resource::Commodity(c) => {
                commodities.insert(c);
            }
            Resource::CommodityList(ls) => {
                for c in ls {
                    commodities.insert(c);
                }
            }
            Resource::Deposit(d) => {
                let tx = CashBalanceTransaction::Deposit(d.detail);
                if let Some(txs) = transactions.get_mut(&d.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(d.port_id, vec![tx]);
                }
            }
            Resource::Withdrawal(w) => {
                let tx = CashBalanceTransaction::Withdraw(w.detail);
                if let Some(txs) = transactions.get_mut(&w.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(w.port_id, vec![tx]);
                }
            }
            Resource::Buy(b) => {
                let tx = CashBalanceTransaction::Buy(b.detail);
                if let Some(txs) = transactions.get_mut(&b.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(b.port_id, vec![tx]);
                }
            }
            Resource::Sell(s) => {
                let tx = CashBalanceTransaction::Sell(s.detail);
                if let Some(txs) = transactions.get_mut(&s.port_id) {
                    txs.push(tx);
                } else {
                    transactions.insert(s.port_id, vec![tx]);
                }
            }
        }
    }

    CategorizedResources {
        portfolios,
        commodities,
        transactions,
    }
}
