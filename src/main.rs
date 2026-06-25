mod cli;
mod commands;
mod services;
mod models;

use clap::Parser;


use crate::{cli::Cli, commands::{add, list, remove, show, watch}};

fn execute(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        cli::Commands::Add(args) => add::run_with_args(args),
        cli::Commands::AddWithArgs(args) => add::run_with_args(args),
        cli::Commands::List => list::run(),
        cli::Commands::Show(args) => show::run(args),
        cli::Commands::Watch => watch::run(),
        cli::Commands::Remove(args) => remove::run(args),
    }
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    execute(cli)
}
