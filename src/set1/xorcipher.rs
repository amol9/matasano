use std::env;
use std::io;

use common::{err, charfreq, challenge};
use common::cipher::one_byte_xor as obx;


pub static info: challenge::Info = challenge::Info {
    no:         2,
    title:      "",
    help:       "",
    execute_fn: interactive
};


pub fn i_generate_base_frequency_file() -> i32 {
    let sample_filepath = match env::args().nth(3) {
        Some(v) => v,
        None    => { println!("please provide the path to sample data file"); return 1; }
    };

    match charfreq::generate_base_frequency_file(&sample_filepath) {
        Ok(_)   => 0,
        Err(_)  => 1
    }
}


pub fn i_decipher() -> i32 {
    println!("enter the hex string to be deciphered: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    match obx::try_decipher(&input.trim()) {
        Ok(v)   => { println!("{}", v.plain); 0 },
        Err(e)  => { println!("{}", e); 1 }
    }
}


pub fn interactive() -> i32 {
    match env::args().nth(2) {
        Some(v) => match v.as_ref() {
                        "genbase"   => i_generate_base_frequency_file(),
                        _           => i_decipher()
                   },
        None    => i_decipher()
    }
}

