use anyhow::{Ok, Result, bail};
use clap::Args;

use crate::{models::account::Account, services::{self, otp::normalize_secret}};

#[derive(Args)]
pub struct AddArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub secret: String,
}

fn validate_secret(secret: &str) -> Result<()> {
    let secret = normalize_secret(secret);
    let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, &secret);

    if decoded.is_none() {
        bail!("Secret is not valid Base32");
    }

    Ok(())
}



#[derive(clap::Args)]
pub struct AddArgsInput {
    pub name: Option<String>,
    pub secret: Option<String>,
}

pub fn run_with_args(args: AddArgs) -> Result<()> {
    validate_secret(&args.secret)?;

    let mut accounts = services::storage::load()?;

    if accounts.iter().any(|a| a.name == args.name) {
        bail!("Account '{}' exist", args.name);
    }

    let account = Account {
        name: args.name,
        secret: args.secret,
    };

    accounts.push(account);
    services::storage::save(&accounts)?;
    Ok(())
}
