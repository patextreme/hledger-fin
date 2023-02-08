use std::path::Path;

use crate::{hledger::HLedgerShow, input, journal};

pub fn print_journal<P: AsRef<Path>>(path: P) {
    let resources = input::from_file(path).unwrap();
    let entries = journal::build_journal(resources);
    for e in entries {
        let s = e.hledger_show();
        println!("{s}\n");
    }
}
