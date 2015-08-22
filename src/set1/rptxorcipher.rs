use std::io;
use std::io::prelude::*;

use common::cipher::rpt_key_xor as rkx;
use common::util;


pub fn interactive() -> u32 {
    let mut plain = String::new();
    input!("enter plain text: ", &mut plain);

    let mut key = String::new();
    input!("enter key: ", &mut key);

    let cipher = rtry!(rkx::encrypt_str(&plain, &key), 1);
    println!("cipher: {}", cipher);
    0
}

