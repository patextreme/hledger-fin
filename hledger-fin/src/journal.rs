use crate::{
    input::Resource,
    inventory::{FifoInventory, Inventory, Lot},
    model::{
        port::CashBalancePortfolio,
        txn::{cashbalance as cb, Buy, DatedTransaction, Deposit, Sell, Withdraw},
        Account, Commodity, CommodityAmount, Date, PortId,
    },
};
use rust_decimal::Decimal;
use std::collections::{hash_map::Entry, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<(Commodity, CommodityAmount)>,
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

    pub fn with_amount<C: Into<Commodity>, Amt: Into<CommodityAmount>>(
        self,
        amount: (C, Amt),
    ) -> Self {
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
            Resource::Withdraw(i) => {
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
                cb::Transaction::Withdraw(t) => result.push(self.generate_withdraw(t)),
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
                    result.extend(self.generate_sell(t, inventory))
                }
            }
        }
        result
    }
}

impl CashBalanceJournalWriter {
    fn generate_deposit(&self, deposit: Deposit) -> JournalEntry {
        let comment = deposit
            .comment
            .as_ref()
            .map(|c| format!(" ({c})"))
            .unwrap_or_default();
        JournalEntry {
            date: deposit.date,
            description: format!("Deposit{comment}"),
            postings: vec![
                Posting::new(&self.port.accounts.cash_account)
                    .with_amount((&self.port.base_currency, &deposit.amount)),
                Posting::new(&self.port.accounts.net_investment_account)
                    .with_amount((&self.port.base_currency, -deposit.amount)),
            ],
            inventory: None,
        }
    }

    fn generate_withdraw(&self, withdraw: Withdraw) -> JournalEntry {
        let comment = withdraw
            .comment
            .as_ref()
            .map(|c| format!(" ({c})"))
            .unwrap_or_default();
        JournalEntry {
            date: withdraw.date,
            description: format!("Withdraw{comment}"),
            postings: vec![
                Posting::new(&self.port.accounts.cash_account)
                    .with_amount((&self.port.base_currency, -&withdraw.amount)),
                Posting::new(&self.port.accounts.net_investment_account)
                    .with_amount((&self.port.base_currency, withdraw.amount)),
            ],
            inventory: None,
        }
    }

    fn generate_buy(&self, buy: Buy, inventory: &mut Box<dyn Inventory>) -> JournalEntry {
        let lot = Lot {
            date: buy.date.clone(),
            price: buy.price.clone(),
            volume: buy.volume.clone(),
        };
        inventory.push(lot);
        let cash_spent: Decimal = (Decimal::NEGATIVE_ONE * &buy.price.0 * &buy.volume.0)
            - buy.commission.clone().unwrap_or_default().0
            - buy.vat.clone().unwrap_or_default().0;
        let comment = buy
            .comment
            .as_ref()
            .map(|c| format!(" ({})", c))
            .unwrap_or_default();
        JournalEntry {
            date: buy.date.clone(),
            description: format!(
                "Buy {} {} @{}{}",
                buy.commodity.0, buy.volume.0, buy.price.0, comment
            ),
            postings: vec![
                Posting::new(&self.port.accounts.position_account)
                    .with_amount((&buy.commodity, buy.volume)),
                Posting::new(&self.port.accounts.cash_account)
                    .with_amount((&self.port.base_currency, cash_spent)),
                Posting::new(&self.port.accounts.commission_account)
                    .with_amount((&self.port.base_currency, buy.commission.unwrap_or_default())),
                Posting::new(&self.port.accounts.vat_account)
                    .with_amount((&self.port.base_currency, buy.vat.unwrap_or_default())),
                Posting::new(&self.port.accounts.conversion_account),
            ],
            inventory: Some(inventory.inventory().clone()),
        }
    }

    fn generate_sell(&self, sell: Sell, inventory: &mut Box<dyn Inventory>) -> Vec<JournalEntry> {
        let used_lots = inventory.pop(&sell.volume);
        let cash_received = (sell.price.0 * sell.volume.0)
            - sell.commission.clone().unwrap_or_default().0
            - sell.vat.clone().unwrap_or_default().0;
        let profit_loss: Decimal = used_lots
            .iter()
            .map(|lot| (sell.price.0 - lot.price.0) * lot.volume.0 * Decimal::NEGATIVE_ONE)
            .sum();
        let profit_loss_comment = used_lots
            .iter()
            .map(|lot| format!("{:?} @{:?}", lot.volume.0, lot.price.0))
            .collect::<Vec<String>>()
            .join(" / ");
        let comment = sell
            .comment
            .as_ref()
            .map(|c| format!(" ({})", c))
            .unwrap_or_default();
        let sell_entry = JournalEntry {
            date: sell.date.clone(),
            description: format!(
                "Sell {} {} @{}{}",
                sell.commodity.0, sell.volume.0, sell.price.0, comment
            ),
            postings: vec![
                Posting::new(&self.port.accounts.position_account)
                    .with_amount((&sell.commodity, -&sell.volume)),
                Posting::new(&self.port.accounts.cash_ar_account)
                    .with_amount((&self.port.base_currency, cash_received)),
                Posting::new(&self.port.accounts.commission_account).with_amount((
                    &self.port.base_currency,
                    &sell.commission.unwrap_or_default(),
                )),
                Posting::new(&self.port.accounts.vat_account)
                    .with_amount((&self.port.base_currency, sell.vat.unwrap_or_default())),
                Posting::new(&self.port.accounts.protfit_loss_account)
                    .with_amount((&self.port.base_currency, profit_loss))
                    .with_comment(profit_loss_comment),
                Posting::new(&self.port.accounts.conversion_account),
            ],
            inventory: Some(inventory.inventory().clone()),
        };
        let settlement_entry = JournalEntry {
            date: sell.settlement_date.unwrap_or_else(|| sell.date),
            description: format!(
                "Settle {} {} @{}",
                sell.commodity.0, sell.volume.0, sell.price.0
            ),
            postings: vec![
                Posting::new(&self.port.accounts.cash_account)
                    .with_amount((&self.port.base_currency, cash_received)),
                Posting::new(&self.port.accounts.cash_ar_account),
            ],
            inventory: None,
        };
        vec![sell_entry, settlement_entry]
    }
}
