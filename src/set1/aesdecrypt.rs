use std::io;
use std::io::prelude::*;
use std::env;

extern crate crypto;
use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::symmetriccipher::Decryptor;
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

use common::{err, util, ascii, base64};


pub fn decrypt_from_file(filepath: &str, key: &str) -> Result<String, err::Error> {
    let cipherbase64 = try!(util::read_file_to_str(&filepath));
    let cipherclean = try!(ascii::filter_whitespace(&cipherbase64));
    let cipherraw = try!(base64::base64_to_raw(&cipherclean));

    let keyraw = try!(ascii::str_to_raw(&key));
    let plainraw = try!(aes_128_ecb_decrypt(&cipherraw, &keyraw));

    let plaintext = try!(ascii::raw_to_str(&plainraw));
    Ok(plaintext)
}


pub fn aes_128_ecb_decrypt(input: &Vec<u8>, key: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
    let mut decryptr = aes::ecb_decryptor(
        aes::KeySize::KeySize128,
        key,
        blockmodes::PkcsPadding);

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(input);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = match decryptr.decrypt(&mut read_buffer, &mut write_buffer, true) {
                Ok(v)   => v,
                Err(_)  => return mkerr!("decryption error")
            };
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        Ok(final_result)
}


pub fn interactive() -> u32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return 1; }
    };

    let mut key = String::new();
    input!("enter key: ", &mut key);

    let plaintext = rtry!(decrypt_from_file(&input_filepath, &key.trim()), 1);
    println!("{}", plaintext);
    0
}

