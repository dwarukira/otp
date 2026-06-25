use clap::Args;
use anyhow::{bail, Result};

use crate::services::storage;

#[derive(Args)]
pub struct RemoveArgs {
    pub name: String,
}

pub fn run(args: RemoveArgs) -> Result<()> {
    let mut accounts = storage::load()?;

    let original_len = accounts.len();

    accounts.retain(|account| {
        account.name != args.name
    });

    if accounts.len() == original_len {
        bail!(
            "Account '{}' not found",
            args.name
        );
    }

    storage::save(&accounts)?;

    println!(
        "✓ Removed '{}'",
        args.name
    );

    Ok(())
}