mod input;
mod inventory;
mod model;

fn main() {
    let resources = input::from_file("./examples/resources.yaml").unwrap();
    for r in resources {
        println!("---");
        println!("{r:?}");
    }
}
