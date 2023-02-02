use std::cmp;

use rust_decimal::Decimal;

use crate::model::{Date, UnitAmount, UnitPrice};

#[derive(Debug, Clone)]
pub struct Lot {
    pub date: Date,
    pub price: UnitPrice,
    pub volume: UnitAmount,
}

pub trait Inventory {
    fn push(&mut self, lot: Lot);
    fn pop(&mut self, volume: &UnitAmount) -> Vec<Lot>;
    fn inventory(&self) -> &Vec<Lot>;
}

#[derive(Debug, Clone, Default)]
pub struct FifoInventory {
    inventory_inner: Vec<Lot>,
}

impl Inventory for FifoInventory {
    fn push(&mut self, lot: Lot) {
        self.inventory_inner.push(lot);
    }

    fn pop(&mut self, volume: &UnitAmount) -> Vec<Lot> {
        let mut used_lots = Vec::new();
        let mut remaining_volume = volume.0;
        for lot in self.inventory() {
            if remaining_volume <= Decimal::ZERO {
                break;
            } else {
                let volume_taken = cmp::min(remaining_volume, lot.volume.0);
                remaining_volume -= volume_taken;
                let used_lot = Lot {
                    volume: UnitAmount(volume_taken),
                    ..lot.clone()
                };
                used_lots.push(used_lot);
            }
        }

        // TODO: properly handle error
        if remaining_volume > Decimal::ZERO {
            panic!("not enough volume to take out from inventory")
        }

        self.inventory_inner = self
            .inventory_inner
            .iter()
            .enumerate()
            .map(|(idx, original_lot)| {
                // TODO: avoid cloning, add performance benchmarking
                if idx >= used_lots.len() {
                    original_lot.clone()
                } else {
                    let used_lot = &used_lots[idx];
                    Lot {
                        volume: &original_lot.volume - &used_lot.volume,
                        ..original_lot.clone()
                    }
                }
            })
            .filter(|i| i.volume.0 > Decimal::ZERO)
            .collect();

        used_lots
    }

    fn inventory(&self) -> &Vec<Lot> {
        &self.inventory_inner
    }
}
