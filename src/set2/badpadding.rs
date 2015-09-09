use std::io;
use std::io::prelude::*;

use common::cipher::padding;
use common::{challenge, err, ascii};


pub static info: challenge::Info = challenge::Info {
    no:         15,
    title:      "PKCS#7 padding validation",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    let mut text = String::new();
    input!("enter padded text (\\x?? for hex value of char): ", &mut text);

    let mut blocksize_str = String::new();
    input!("enter block size [16]: ", &mut blocksize_str);

    let blocksize = match blocksize_str.trim().as_ref() {
        ""  => 16,
        _   => rtry!(blocksize_str.trim().parse::<usize>(), exit_err!())
    };

    text = rtry!(ascii::scan_hex(&text), exit_err!());
    let raw = rtry!(ascii::str_to_raw(&text.trim()), exit_err!());
    println!("raw len: {}", raw.len());

    match padding::pkcs7_detect(&raw, blocksize) {
        Ok(v)  => println!("padding length = {}", v),
        Err(e) => println!("{}", e)
    }
    exit_ok!()
}

