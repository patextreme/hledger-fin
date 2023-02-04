mod hledger;
mod input;
mod inventory;
mod journal;
mod model;

fn main() {
    let resources = input::from_file("./examples/resources.yaml").unwrap();
    journal::build_journal(resources);
}
