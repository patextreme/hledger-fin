use crate::journal::JournalEntry;
use rust_decimal::Decimal;

pub trait HLedgerShow {
    fn hledger_show(&self) -> String;
}

impl HLedgerShow for JournalEntry {
    fn hledger_show(&self) -> String {
        let inventory = self.inventory.as_ref().map_or("".into(), |lots| {
            let inventory_str = lots
                .iter()
                .map(|lot| format!("{} @{}", lot.volume.0, lot.price.0))
                .collect::<Vec<String>>()
                .join(", ");
            let total_lot: Decimal = lots.iter().map(|lot| lot.volume.0).sum();
            let avg_cost = if total_lot == Decimal::ZERO {
                Decimal::ZERO
            } else {
                lots.iter()
                    .map(|lot| lot.volume.0 * lot.price.0)
                    .sum::<Decimal>()
                    / total_lot
            };
            let avg_cost = avg_cost.round_dp(6);
            format!(
                "  ; avg {} @{} ; inventory [{}]",
                total_lot, avg_cost, inventory_str
            )
        });
        let max_account_len = self
            .postings
            .iter()
            .map(|p| p.account.0.len())
            .max()
            .unwrap_or_default();
        let postings = self
            .postings
            .iter()
            .map(|p| {
                let amount_str = p
                    .amount
                    .as_ref()
                    .map(|a| format!("{} {}", a.0 .0, a.1 .0))
                    .unwrap_or_else(|| "".into());
                let comment_str = p
                    .comment
                    .as_ref()
                    .map(|c| format!("  ; {}", c))
                    .unwrap_or_else(|| "".into());
                let spaces = " ".repeat(max_account_len + 4 - p.account.0.len());
                format!("    {}{}{}{}", p.account.0, spaces, amount_str, comment_str)
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "{date} {desc}{inventory}\n{postings}",
            date = self.date.0,
            desc = self.description,
            postings = postings,
            inventory = inventory
        )
    }
}
