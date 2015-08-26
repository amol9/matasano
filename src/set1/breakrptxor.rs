use std::io;
use std::io::prelude::*;
use std::env;
use std::ops;
use std::slice;

use common::{err, util, ascii, base64};
use common::cipher::one_byte_xor as obx;
use common::cipher::rpt_key_xor as rkx;


// read input cipher text file
// compute key length
// transpose input cipher
// for each row, compute key byte
// combine to make full key
// decrypt the cipher and print plain text

const KEYLENGTH_RANGE: ops::Range<u8> = ops::Range {start: 1, end: 40};


pub fn break_cipher(filepath: &str) -> Result<String, err::Error> {
    let cipherbase64 = try!(util::read_file_to_str(&filepath));
    let cipherclean = try!(ascii::filter_whitespace(&cipherbase64));
    let cipherraw = try!(base64::base64_to_raw(&cipherclean));

    let keylength = try!(guess_key_length(&cipherraw));
    println!("keylength = {}", keylength);

    let key = try!(guess_key(&cipherraw, keylength));
    println!("key = {}", try!(ascii::raw_to_str(&key)));

    let plaintext = try!(rkx::decrypt_raw(&cipherraw, &key));

    Ok(try!(ascii::raw_to_str(&plaintext)))
}


pub fn guess_key_length(cipherraw: &Vec<u8>) -> Result<u8, err::Error> {
    let mut dist: Vec<f32> = Vec::new();

    for block_len in KEYLENGTH_RANGE {
        //let cipherbytes: &[u8] = ciphertext.as_ref(); 
        let mut blocks: slice::Chunks<u8> = cipherraw.chunks(block_len as usize);
        let mut d_avg: Option<f32> = None;

        while true {
            let b1 = match blocks.next() {
                Some(v) => v.to_vec(),
                None    => break
            };

            let b2 = match blocks.next() {
                Some(v) => v.to_vec(),
                None    => break
            };

            if b1.len() != b2.len() {
                break;
            }

            let d = etry!(util::hamm_vec(&b1, &b2), "hamming distance calculation error");
            d_avg = match d_avg {
                Some(v) => Some((v + d as f32) / 2 as f32),
                None    => Some(d as f32)
            };
        }
        dist.push(d_avg.unwrap() as f32 / block_len as f32);
    }
    let keylength: u8 = util::min_index(&dist).unwrap() as u8 + KEYLENGTH_RANGE.start;
    Ok(keylength)
}


pub fn guess_key(cipherraw: &Vec<u8>, keylength: u8) -> Result<Vec<u8>, err::Error> {
    //let cipher_trans: Vec<Vec<char>> = etry!(util::transpose_vec::<char>(&ciphertext.chars().collect(), keylength as u32),
    //                                    "cipher text transposing error");
    let cipher_trans = etry!(util::transpose_vec(&cipherraw, keylength as u32), "transpose error");
    println!("cipher trans vec len = {}", cipher_trans.len());

    let mut key = Vec::new();
    for slice in cipher_trans {
        let keybyte: u8 = try!(obx::guess_key(&slice)).key;
        println!("key byte: {}", keybyte);
        key.push(keybyte);
    }
    Ok(key)
}


pub fn interactive() -> u32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return 1; }
    };

    let plaintext = rtry!(break_cipher(&input_filepath), 1);
    println!("{}", plaintext);
    0
}

