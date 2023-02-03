use crate::{
    inventory::Lot,
    model::{Account, Commodity, Date, UnitAmount},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<(Commodity, UnitAmount)>,
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

    pub fn with_amount<C: Into<Commodity>, Amt: Into<UnitAmount>>(self, amount: (C, Amt)) -> Self {
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
