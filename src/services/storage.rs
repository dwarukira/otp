use anyhow::{Ok, Result};

// use std::env;
use std::fs;
use std::path::PathBuf;

use crate::models::account::Account;

fn file_path() -> Result<PathBuf> {
    let mut path = PathBuf::from(
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE")) // fallback for Windows
            .map_err(|_| anyhow::anyhow!("Could not find home directory"))?,
    );

    path.push(".config");
    path.push("otp");
    fs::create_dir_all(&path)?;
    path.push("accounts.json");

    Ok(path)
}

pub fn load() -> Result<Vec<Account>> {
    let path = file_path()?;

    if !path.exists() {
        return Ok(vec![]);
    }

    let contents = fs::read_to_string(path)?;

    Ok(serde_json::from_str(&contents)?)
}


pub fn save(
    accounts: &[Account]
) -> Result<()> {
    let path = file_path()?;
    let json = serde_json::to_string_pretty(accounts)?;

    fs::write(path, json)?;

    Ok(())
}