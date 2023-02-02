use crate::model::{Date, UnitAmount, UnitPrice};

#[derive(Debug, Clone)]
pub struct Lot {
    pub date: Date,
    pub price: UnitPrice,
    pub volume: UnitAmount,
}

pub trait Inventory {
    fn push(&mut self, lot: &Lot);
    fn pop(&mut self, volume: &UnitAmount) -> Vec<Lot>;
    fn inventory(&self) -> &Vec<Lot>;
}
