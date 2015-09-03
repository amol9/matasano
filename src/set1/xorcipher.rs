use std::env;
use std::io;
use std::io::Write;

use common::{err, charfreq, challenge};
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

    match obx::try_decipher(&input.trim()) {
        Ok(v)   => { println!("{}", v.plain); exit_ok!() },
        Err(e)  => { println!("{}", e); exit_err!() }
    }
}

