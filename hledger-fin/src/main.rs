use crate::input::Resource;
use serde::Deserialize;
use serde_yaml::Value;
use std::fs;

mod input;
mod model;

fn main() {
    let input_yaml = fs::read_to_string("./examples/resources.yaml").unwrap();
    for doc in serde_yaml::Deserializer::from_str(&input_yaml) {
        let value = Value::deserialize(doc).unwrap();
        let resource: Resource = serde_yaml::from_value(value).unwrap();
        println!("---");
        println!("{resource:?}");
    }
}
