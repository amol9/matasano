extern crate matasano;
use self::matasano::common::{util, ascii};
use self::matasano::set1::breakrptxor as brx;
use self::matasano::common::cipher::rpt_key_xor as rkx;


#[test]
fn test_cryptopals_case() {
    let data_filepath = "data/6.txt";
    let guess = m!(brx::break_cipher_from_file(&data_filepath));

    println!("key: {}\n\n {}", guess.key, guess.text);
    assert_eq!(guess.key, "Terminator X: Bring the noise");
}


//#[test]
fn test_break() {
    fn enc(input: &str, key: &str) -> Option<Vec<u8>> {
        let plain = mr!(ascii::str_to_raw(&input), None);
        let key = mr!(ascii::str_to_raw(&key), None);
        let cipher = mr!(rkx::encrypt_raw(&plain, &key), None);
        Some(cipher)
    }

    let text = m!(util::read_file_to_str("data/prob-6.txt"));
    let cipher = enc(&text, "key").unwrap();
    let guess = m!(brx::break_cipher(&cipher));

    println!("key: {}\n\n {}", guess.key, guess.text);
}

