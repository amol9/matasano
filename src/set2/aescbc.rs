use std::io;
use std::io::prelude::*;
use std::env;

use common::{err, util, ascii, base64};
use common::cipher::aes;

const challenge_no: u8 = 10;
const challenge_title: &str = "Implement CBC mode";



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

