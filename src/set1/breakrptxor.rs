use std::io;
use std::io::prelude::*;

use common::{err, cipher, util};


// read input cipher text file
// compute key length
// transpose input cipher
// for each row, compute key byte
// combine to make full key
// decrypt the cipher and print plain text

const KEYLENGTH_RANGE: Range<u8> = (1..40);


pub fn break_cipher(filepath: &str) -> Result<String, err::Error> {
    let ciphertext = try!(ascii::read_file_to_str(&filepath));

    let keylength = try!(compute_key_length(&ciphertext));

    let key = try!(guess_key(&ciphertext, keylength));

    let plaintext = try!(rkx::decrypt_str(&ciphertext, &key));

    Ok(plaintext)
}


//check and fix
pub fn compute_key_length(ciphertext: &str) -> Result<u8, err::Error> {
    let dist: Vec<f32> = Vec::new();

    for block_len in KEYLENGTH_RANGE {
        let blocks = ciphertext.chunks(block_len);
        let d_total = 0f32;
        for block_no in (1 .. (blocks.len())).step_by(2) {
             d_total += util::hamm_vec(blocks.next().unwrap(), blocks.next().unwrap());
        }
        dist.push(d_total / block_len);
    }
    let keylength = try!(util::min_index(&dist)) as u8 + KEYLENGTH_RANGE.start;
    Ok(keylength)
}


pub fn guess_key(ciphertext: &str, keylength: u8) -> Result<String, err::Error> {
    let cipher_trans = etry!(util::transpose_iter(ciphertext.iter(), keylength), "cipher text transposing error");

    let key: String = cipher_trans.iter().map(|ch| ascii::u8_to_char(try!(obx::guess_key(&ch.into_iter().collect()).key))).collect();
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

