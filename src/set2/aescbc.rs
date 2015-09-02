use std::io;
use std::io::prelude::*;
use std::env;

use common::{err, util, ascii, base64};
use common::cipher::aes;

const challenge_no: u8 = 10;
const challenge_title: &'static str = "Implement CBC mode";


pub fn decrypt_from_file(filepath: &str, key: &str) -> Result<String, err::Error> {
    let cipherbase64 = try!(util::read_file_to_str(&filepath));
    let cipherclean = try!(ascii::filter_whitespace(&cipherbase64));
    let cipherraw = try!(base64::base64_to_raw(&cipherclean));

    let keyraw = try!(ascii::str_to_raw(&key));
    let plainraw = try!(aes::decrypt(&cipherraw, &keyraw, &aes::cbc_128_pkcs7));

    let plaintext = try!(ascii::raw_to_str(&plainraw));
    Ok(plaintext)
}


pub fn interactive() -> u32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return 1; }
    };

    let mut key = String::new();
    input!("enter key: ", &mut key);

    let plaintext = rtry!(decrypt_from_file(&input_filepath, &key.trim()), 1);
    println!("{}", plaintext);
    0
}
