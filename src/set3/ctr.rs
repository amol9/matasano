use std::io;
use std::io::prelude::*;

use common::{err, challenge, ascii, base64};
use common::cipher::aes;



pub static info: challenge::Info = challenge::Info {
    no:         18,
    title:      "Implement CTR, the stream cipher mode",
    help:       "",
    execute_fn: interactive
};


pub fn ctr_crypt(input_b64: &str, key: &str) -> Result<String, err::Error> {
    let mut ctr = aes::CTR::new(&raw!(&key), 0);
    Ok(rts!(&try!(ctr.gen(&b64d!(&input_b64)))))
}


pub fn interactive() -> err::ExitCode {
    let mut input_b64 = String::new();
    input!("enter input (base64): ", &mut input_b64);
    
    let mut key = String::new();
    input!("enter key", &mut key, "YELLOW SUBMARINE");

    let output = rtry!(ctr_crypt(&input_b64.trim(), &key.trim()), exit_err!());
    println!("{}", output);
   
    exit_ok!()
}
