use std::io;
use std::io::prelude::*;

use common::cipher::padding;


pub fn interactive() -> u32 {
    let mut text = String::new();
    input!("enter text: ", &mut text);

    let mut bsize = String::new();
    input!("enter block size: ", &mut bsize);

    let blocksize = match bsize.trim().parse::<usize>() {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); return 1; }
    };

    text = String::from(text.trim());
    rtry!(padding::pkcs7(&mut text, blocksize), 1);

    padding::print_pkcs7(&text, blocksize);
    0
}