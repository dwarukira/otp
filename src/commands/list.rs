use anyhow::{Ok, Result};

use crate::services::storage;

pub fn run() -> Result<()> {
    let accounts = storage::load()?;

    if accounts.is_empty() {
        println!("No accounts");

        return Ok(());
    }

    for account in accounts {
        println!("{}", account.name);
    }

    Ok(())
}
