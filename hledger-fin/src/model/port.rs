use super::{Account, Commodity, PortId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashBalancePortfolio {
    pub port_id: PortId,
    pub base_currency: Commodity,
    pub accounts: CashBalancePortfolioAccounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashBalancePortfolioAccounts {
    pub cash_account: Account,
    pub cash_ar_account: Account,
    pub position_account: Account,
    pub net_investment_account: Account,
    pub conversion_account: Account,
    pub commission_account: Account,
    pub vat_account: Account,
    pub protfit_loss_account: Account,
}
