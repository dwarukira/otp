use anyhow::{Ok, Result, bail};
use clap::Args;

use crate::services;

#[derive(Args)]
pub struct ShowArgs {
    pub name: String,
}

pub fn run(args: ShowArgs) -> Result<()> {
    let accounts = services::storage::load()?;

    let account = accounts.iter().find(|a| a.name == args.name);

    let account = match account {
        Some(a) => a,
        None => bail!("Account not found"),
    };

    let code = services::otp::generate(&account.secret)?;

    println!("{}", code);

    Ok(())
}
