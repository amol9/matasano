
use common::{err, hex};


pub fn encrypt_str(plain: &str, key: &str) -> Result<String, err::Error> {
    Ok(etry!(hex::raw_to_hex::<hex::lower>(&plain.chars().zip(key.chars().cycle()).map(|(a, b)| a as u8 ^ b as u8).collect()),
        "encryption error"))
}

