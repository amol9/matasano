
use common::{err, hex, ascii};


pub fn encrypt_str(plain: &str, key: &str) -> Result<String, err::Error> {
    Ok(etry!(hex::raw_to_hex::<hex::lower>(&plain.chars().zip(key.chars().cycle()).map(|(a, b)| a as u8 ^ b as u8).collect()),
        "encryption error"))
}


pub fn decrypt_str(cipher: &str, key: &str) -> Result<String, err::Error> {
    let plain: String = cipher.chars().zip(key.chars().cycle()).map(|(a, b)| ascii::u8_to_char(a as u8 ^ b as u8)).collect();
    Ok(plain)
}

