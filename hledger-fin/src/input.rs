use crate::model::{Commodity, spot::SpotPortfolio};

#[derive(Debug, Clone)]
pub enum DeclaredResource {
    Commodity(Commodity),
    SpotPortfolio(SpotPortfolio),
}
