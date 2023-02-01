use crate::model::{
    port::CashBalancePortfolio,
    txn::{Buy, Deposit, Sell, Withdrawal},
    Commodity,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "spec")]
pub enum Resource {
    Commodity(Commodity),
    CommodityList(Vec<Commodity>),
    CashBalancePortfolio(Box<CashBalancePortfolio>),
    Deposit(PortfolioScopedResource<Deposit>),
    Withdrawal(PortfolioScopedResource<Withdrawal>),
    Buy(PortfolioScopedResource<Buy>),
    Sell(PortfolioScopedResource<Sell>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioScopedResource<T> {
    pub port_id: String,
    pub detail: T,
}
