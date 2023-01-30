use crate::model::{port::CashBalancePortfolio, Commodity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "spec")]
pub enum DeclaredResource {
    Commodity(Commodity),
    CommodityList(Vec<Commodity>),
    CashBalancePortfolio(CashBalancePortfolio),
}
