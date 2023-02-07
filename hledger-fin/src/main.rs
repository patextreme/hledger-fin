use crate::hledger::HLedgerShow;

mod hledger;
mod input;
mod inventory;
mod journal;
mod model;

fn main() {
    let resources = input::from_file("./examples/sample.yaml").unwrap();
    let entries = journal::build_journal(resources);
    for e in entries {
        let s = e.hledger_show();
        println!("{s}\n");
    }
}
