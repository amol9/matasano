use std::env;

use common::{err, util, hex, challenge};
use common::cipher::oracle;


pub static info: challenge::Info = challenge::Info {
    no:         1,
    title:      "",
    help:       "",
    execute_fn: interactive
};


const BLOCK_SIZE: usize = 16;


pub fn detect_from_list(filepath: &str) -> Result<Vec<String>, err::Error> {
    let input = try!(util::read_file_to_str(&filepath));
    let mut ciphers = Vec::new();

    for line in input.lines() {
        ciphers.push(try!(hex::hex_to_raw(line)));
    }

    let mut citer = input.lines();
    let mut result = Vec::new();

    for cipherraw in ciphers {
        let c = citer.next().unwrap();
        match oracle::detect_aes_ecb(&cipherraw, BLOCK_SIZE) {
            Ok(v)   => match v {
                true    => result.push(String::from(c)),
                false   => {}
                },
            Err(e)  => println!("error: {}\nblock: {}", e, c)
        };
    }
    Ok(result)
}


pub fn interactive() -> i32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return 1; }
    };

    let result = rtry!(detect_from_list(&input_filepath), 1);
    for r in result {
        println!("{}\n", r);
    }
    0
}

