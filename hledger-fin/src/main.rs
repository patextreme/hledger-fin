use clap::Parser;

mod cli;
mod command;
mod hledger;
mod input;
mod inventory;
mod journal;
mod model;

fn main() {
    let cli = cli::Cli::parse();
    cli.exec();
}
