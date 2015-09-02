use std::io;
use std::fs;
use std::env;
use std::f32;

use common::{err, util, challenge};
use common::cipher::one_byte_xor as obx;


pub static info: challenge::Info = challenge::Info {
    no:         5,
    title:      "",
    help:       "",
    execute_fn: interactive
};


pub fn read_input_file(filepath: &str) -> Result<Vec<String>, err::Error> {
    let text = try!(util::read_file_to_str(&filepath));

    let mut output = Vec::new();
    for line in text.split('\n') {
        output.push(String::from(line.trim()));
    }
    Ok(output)
}


pub fn detect_xor_cipher(input: &Vec<String>) -> Result<String, err::Error> {
    let mut dist: Vec<f32> = Vec::new();

    for hexstr in input {
        match obx::try_decipher(&hexstr) {
            Ok(v)   => dist.push(v.distance),
            Err(_)  => dist.push(f32::MAX)
        };
    }

    let cipher_index = util::min_index::<f32>(&dist).unwrap();
    Ok(input[cipher_index].clone())
}


pub fn interactive() -> i32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return 1; }
    };

    let input = rtry!(read_input_file(&input_filepath), 1);
    let output = rtry!(detect_xor_cipher(&input), 1);
    println!("cipher string: {}", output);
    0
}

