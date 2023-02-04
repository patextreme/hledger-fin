use serde::{Deserialize, Serialize};

use super::{CashAmount, Commodity, Date, UnitAmount, UnitPrice};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    pub date: Date,
    pub amount: CashAmount,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Withdraw {
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
    pub commission: Option<CashAmount>,
    pub vat: Option<CashAmount>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sell {
    pub date: Date,
    pub settlement_date: Option<Date>,
    pub commodity: Commodity,
    pub price: UnitPrice,
    pub volume: UnitAmount,
    pub commission: Option<CashAmount>,
    pub vat: Option<CashAmount>,
    pub comment: Option<String>,
}

pub trait DatedTransaction {
    fn date(&self) -> &Date;
}

impl DatedTransaction for Deposit {
    fn date(&self) -> &Date {
        &self.date
    }
}

impl DatedTransaction for Withdraw {
    fn date(&self) -> &Date {
        &self.date
    }
}

impl DatedTransaction for Buy {
    fn date(&self) -> &Date {
        &self.date
    }
}

impl DatedTransaction for Sell {
    fn date(&self) -> &Date {
        &self.date
    }
}
