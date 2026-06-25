use anyhow::{Ok, Result, anyhow};
use base32::Alphabet;
use totp_rs::{Algorithm, TOTP};


pub fn normalize_secret(secret: &str) -> String {
    secret
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_uppercase()
}


pub fn generate(secret: &str) -> Result<String> {
      let secret = normalize_secret(secret);

    let secret_bytes = base32::decode(Alphabet::Rfc4648 { padding: false }, &secret)
        .ok_or_else(|| anyhow!("Invalid Base32 secret"))?;

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret_bytes)?;

    let code = totp.generate_current()?;

    Ok(code)
}

