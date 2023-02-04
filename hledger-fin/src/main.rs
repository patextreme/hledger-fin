mod command;
mod hledger;
mod input;
mod inventory;
mod model;

fn main() {
    let resources = input::from_file("./examples/resources.yaml").unwrap();
    for r in resources.iter() {
        println!("---");
        println!("{r:?}");
    }

    command::build_journal(resources);
}
