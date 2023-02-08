use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::command;

#[derive(Subcommand)]
pub enum Command {
    /// Print journal
    Print,
}

#[derive(Parser)]
pub struct Cli {
    /// Set input resource file
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn exec(self) {
        match self.command {
            Command::Print => command::print_journal(self.file),
        }
    }
}
