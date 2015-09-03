use std::io;
use std::io::prelude::*;

use common::cipher::rpt_key_xor as rkx;
use common::{util, challenge, err};


pub static info: challenge::Info = challenge::Info {
    no:         5,
    title:      "Implement repeating-key XOR",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    let mut plain = String::new();
    input!("enter plain text: ", &mut plain);

    let mut key = String::new();
    input!("enter key: ", &mut key);

    let cipher = rtry!(rkx::encrypt_str(&plain, &key), exit_err!());
    println!("cipher: {}", cipher);
    exit_ok!()
}

