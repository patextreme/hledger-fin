use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::{
    input::Resource,
    inventory::{FifoInventory, Inventory, Lot},
    model::{
        port::CashBalancePortfolio,
        txn::{cashbalance as cb, Buy, DatedTransaction, Deposit, Sell, Withdraw},
        Account, Commodity, Date, PortId, UnitAmount,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<(Commodity, UnitAmount)>,
    pub comment: Option<String>,
}

impl Posting {
    pub fn new(account: impl Into<Account>) -> Self {
        Self {
            account: account.into(),
            amount: None,
            comment: None,
        }
    }

    pub fn with_amount<C: Into<Commodity>, Amt: Into<UnitAmount>>(self, amount: (C, Amt)) -> Self {
        Self {
            amount: Some((amount.0.into(), amount.1.into())),
            ..self
        }
    }

    pub fn with_comment(self, comment: impl Into<String>) -> Self {
        Self {
            comment: Some(comment.into()),
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub date: Date,
    pub description: String,
    pub postings: Vec<Posting>,
    pub inventory: Option<Vec<Lot>>,
}

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
            let writer = CashBalanceJournalWriter { port };
            let entries = writer.to_journal_entries(transactions);
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

trait JournalWriter<T> {
    fn to_journal_entries(&self, transactions: Vec<T>) -> Vec<JournalEntry>;
}

struct CashBalanceJournalWriter {
    port: CashBalancePortfolio,
}

impl JournalWriter<cb::Transaction> for CashBalanceJournalWriter {
    fn to_journal_entries(&self, transactions: Vec<cb::Transaction>) -> Vec<JournalEntry> {
        let mut sorted_transaction: Vec<(usize, cb::Transaction)> =
            transactions.into_iter().enumerate().collect();
        sorted_transaction.sort_by_key(|i| (i.1.date().clone(), i.0));

        let mut inventories: HashMap<Commodity, Box<dyn Inventory>> = HashMap::new();
        let mut result: Vec<JournalEntry> = Vec::new();
        for (_, txn) in sorted_transaction {
            match txn {
                cb::Transaction::Deposit(t) => result.push(self.generate_deposit(t)),
                cb::Transaction::Withdraw(t) => result.extend(self.generate_withdraw(t)),
                cb::Transaction::Buy(t) => {
                    let inventory = match inventories.entry(t.commodity.clone()) {
                        Entry::Occupied(e) => e.into_mut(),
                        Entry::Vacant(e) => e.insert(Box::new(FifoInventory::default())), // TODO: support other cost basis
                    };
                    result.push(self.generate_buy(t, inventory));
                }
                cb::Transaction::Sell(t) => {
                    let inventory = match inventories.entry(t.commodity.clone()) {
                        Entry::Occupied(e) => e.into_mut(),
                        Entry::Vacant(e) => e.insert(Box::new(FifoInventory::default())), // TODO: support other cost basis
                    };
                    result.push(self.generate_sell(t, inventory))
                }
            }
        }
        result
    }
}

impl CashBalanceJournalWriter {
    fn generate_deposit(&self, deposit: Deposit) -> JournalEntry {
        todo!()
    }

    fn generate_withdraw(&self, withdraw: Withdraw) -> Vec<JournalEntry> {
        todo!()
    }

    fn generate_buy(&self, buy: Buy, inventory: &mut Box<dyn Inventory>) -> JournalEntry {
        todo!()
    }

    fn generate_sell(&self, sell: Sell, inventory: &mut Box<dyn Inventory>) -> JournalEntry {
        todo!()
    }
}
