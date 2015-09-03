use std::io;
use std::io::prelude::*;

use common::cipher::padding;
use common::{challenge, err};


pub static info: challenge::Info = challenge::Info {
    no:         9,
    title:      "Implement PKCS#7 padding",
    help:       "",
    execute_fn: interactive
};



pub fn interactive() -> err::ExitCode {
    let mut text = String::new();
    input!("enter text: ", &mut text);

    let mut bsize = String::new();
    input!("enter block size: ", &mut bsize);

    let blocksize = match bsize.trim().parse::<usize>() {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); return exit_err!(); }
    };

    text = String::from(text.trim());
    rtry!(padding::pkcs7(&mut text, blocksize), exit_err!());

    padding::print_pkcs7(&text, blocksize);
    exit_ok!()
}
