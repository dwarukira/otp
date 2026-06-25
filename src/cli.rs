use clap::{Parser, Subcommand};

use crate::commands::{add, remove, show};


#[derive(Parser)]
#[command(name = "otp")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]

pub enum Commands {
    AddWithArgs(add::AddArgs),
    Add(add::AddArgs),
    List,
    Show(show::ShowArgs),
    Watch,
    Remove(remove::RemoveArgs)
}
