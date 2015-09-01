use std::io;
use std::io::prelude::*;

extern crate rand;
use self::rand::Rng;

use common::err;
use common::cipher::{aes, oracle, key};

// fn: to generate random keys
// fn: to generate random data of spec length
// move: detect function to common
// loop: a spec number of samples


pub fn gen_cipher(input: &Vec<u8>, mode: &aes::Mode) -> Result<Vec<u8>, err::Error> {
    let mut rng = rand::thread_rng();
    let mut rand_data = || (0 .. rng.gen::<usize>() % 5).map(|_| rng.gen::<u8>()).collect();

    let mut r_input: Vec<u8> = rand_data();
    r_input.extend(input);
    r_input.extend(&rand_data());

    let key = try!(key::random(mode.blocksize));

    aes::encrypt(&r_input, &key, &mode)
}


pub fn detect_aes_mode(sample_count: usize) -> Result<(usize, usize), err::Error> {
    let aes_modes = [aes::ecb_128_pkcs7, aes::cbc_128_pkcs7];
    let input: Vec<u8> = vec![65; 64];

    let mut success: usize = 0;
    let mut failure: usize = 0;

    let mut rng = rand::thread_rng();

    for _ in 0 .. sample_count {
        let mode: &aes::Mode = &aes_modes[rng.gen::<usize>() % 2];

        let cipher = try!(gen_cipher(&input, &mode));

        let d_mode: aes::Mode;
        if try!(oracle::detect_aes_ecb(&cipher, mode.blocksize)) {
            d_mode = aes::ecb_128_pkcs7;
            println!("ecb");
        } else {
            d_mode = aes::cbc_128_pkcs7;
            println!("cbc");
        }
        
        if d_mode.blockmode == mode.blockmode {
            success += 1;
        } else {
            failure += 1;
        }
    }
    Ok((success, failure))
}


pub fn interactive() -> u32 {
    let mut s = String::new();
    input!("enter sample count: ", &mut s);

    let sample_count = rtry!(s.trim().parse::<usize>(), 1);
    let (success, failure) = rtry!(detect_aes_mode(sample_count), 1);

    println!("tried {} samples: success: {}, failure: {}", sample_count, success, failure);
    0
}

