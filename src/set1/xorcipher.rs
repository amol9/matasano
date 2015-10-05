use std::env;
use std::io;
use std::io::Write;

use common::{err, charfreq, challenge, ascii, hex};
use common::cipher::one_byte_xor as obx;


pub static info: challenge::Info = challenge::Info {
    no:         3,
    title:      "Single-byte XOR cipher",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    let mut input = String::new();
    input!("enter the hex string to be deciphered: ", &mut input);

    match obx::guess_key(&rtry!(hex::hex_to_raw(input.trim()), exit_err!()), None) {
        Ok(v)   => {
            let p = rtry!(ascii::raw_to_str(&v.plain), exit_err!());
            println!("{}", p);
            exit_ok!()
            },
        Err(e)  => {
            println!("{}", e);
            exit_err!()
        }
    }
}

