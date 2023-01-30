use super::{Account, Commodity};

#[derive(Debug, Clone)]
pub struct SpotPortfolio {
    pub name: String,
    pub description: Option<String>,
    pub default_commodity: Commodity,
    pub bookkeeping_config: SpotPortfolioBookkeepingConfig,
}

#[derive(Debug, Clone)]
pub struct SpotPortfolioBookkeepingConfig {
    pub position_acct: Account,
    pub net_investment_acct: Account,
    pub conversion_acct: Account,
    pub commission_acct: Account,
    pub vat_acct: Account,
    pub protfit_loss_acct: Account,
}
