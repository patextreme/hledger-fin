use serde::{Deserialize, Serialize};

use super::{CashAmount, Commodity, Date, UnitAmount, UnitPrice};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub date: Date,
    pub amount: CashAmount,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    pub date: Date,
    pub amount: CashAmount,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buy {
    pub date: Date,
    pub commodity: Commodity,
    pub price: UnitPrice,
    pub volume: UnitAmount,
    pub commission: CashAmount,
    pub vat: CashAmount,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sell {
    pub date: Date,
    pub settlement_date: Option<Date>,
    pub commodity: Commodity,
    pub price: UnitPrice,
    pub volume: UnitAmount,
    pub commission: CashAmount,
    pub vat: CashAmount,
    pub comment: Option<String>,
}