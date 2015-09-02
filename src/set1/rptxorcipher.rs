use std::io;
use std::io::prelude::*;

use common::cipher::rpt_key_xor as rkx;
use common::{util, challenge};


pub static info: challenge::Info = challenge::Info {
    no:         1,
    title:      "",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> i32 {
    let mut plain = String::new();
    input!("enter plain text: ", &mut plain);

    let mut key = String::new();
    input!("enter key: ", &mut key);

    let cipher = rtry!(rkx::encrypt_str(&plain, &key), 1);
    println!("cipher: {}", cipher);
    0
}

