use crate::model::{
    port::CashBalancePortfolio,
    txn::{Buy, Deposit, InterestPayment, Sell, Withdraw},
    PortId,
};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum ImportError {
    #[error("unable to read file: {0}")]
    FileIO(#[from] std::io::Error),
    #[error("unable to parse resource: {0}")]
    ParseError(String),
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Resource>, ImportError> {
    let yaml = std::fs::read_to_string(path.as_ref())?;
    let mut resources = Vec::new();
    for doc in serde_yaml::Deserializer::from_str(&yaml) {
        let value = Value::deserialize(doc).map_err(|e| ImportError::ParseError(e.to_string()))?;
        let r: Resource =
            serde_yaml::from_value(value).map_err(|e| ImportError::ParseError(e.to_string()))?;
        resources.push(r)
    }
    Ok(resources)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "spec")]
pub enum Resource {
    CashBalancePortfolio(Box<CashBalancePortfolio>),
    Deposit(PortfolioScopedResource<Deposit>),
    Withdraw(PortfolioScopedResource<Withdraw>),
    Buy(PortfolioScopedResource<Buy>),
    Sell(PortfolioScopedResource<Sell>),
    Interest(PortfolioScopedResource<InterestPayment>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioScopedResource<T> {
    pub port_id: PortId,
    pub detail: T,
}
