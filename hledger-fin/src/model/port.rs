use super::{Account, Commodity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashBalancePortfolio {
    pub name: String,
    pub base_currency: Commodity,
    pub accounts: CashBalancePortfolioBookkeepingAccounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashBalancePortfolioBookkeepingAccounts {
    pub cash_account: Account,
    pub cash_ar_accont: Account,
    pub position_account: Account,
    pub net_investment_account: Account,
    pub conversion_account: Account,
    pub commission_account: Account,
    pub vat_account: Account,
    pub protfit_loss_account: Account,
}
