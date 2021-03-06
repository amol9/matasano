use std::env;

use common::{err, util, ascii, base64, challenge};
use common::cipher::aes;


pub static info: challenge::Info = challenge::Info {
    no:         7,
    title:      "AES in ECB mode",
    help:       "param1: path to aes ecb encrypted file in base64 form",
    execute_fn: interactive
};


pub fn decrypt_from_file(filepath: &str, key: &str) -> Result<String, err::Error> {
    let cipherbase64 = try!(util::read_file_to_str(&filepath));
    let cipherclean = try!(ascii::filter_whitespace(&cipherbase64));
    let cipherraw = try!(base64::base64_to_raw(&cipherclean));

    let keyraw = try!(ascii::str_to_raw(&key));
    let plainraw = try!(aes::decrypt(&cipherraw, &keyraw, &aes::ecb_128_pkcs7));

    let plaintext = try!(ascii::raw_to_str(&plainraw));
    Ok(plaintext)
}


pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return exit_err!(); }
    };

    let key = rtry!(util::input("enter key"), exit_err!());

    let plaintext = rtry!(decrypt_from_file(&input_filepath, &key.trim()), 1);

    println!("{}", plaintext);

    exit_ok!()
}

