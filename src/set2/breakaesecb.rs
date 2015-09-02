use std::env;
use std::iter::FromIterator;
use std::io;
use std::io::Write;

use common::{err, ascii, base64, util};
use common::cipher::{aes, oracle, key, padding};


const max_blocksize: usize = 32;


pub struct CipherBox {
    key:    Vec<u8>,
    data:   Vec<u8>,
    mode:   aes::Mode
}


impl CipherBox {
    fn new(data: &Vec<u8>, mode: aes::Mode) -> Result<Self, err::Error> {
        Ok(CipherBox {
            key:    try!(key::random(mode.blocksize)),
            data:   data.clone(),
            mode:   mode
        })
    }

    fn gen(&self, prefix: &str) -> Result<Vec<u8>, err::Error> {
        let mut final_input = try!(ascii::str_to_raw(&prefix));
        final_input.extend(&self.data);

        aes::encrypt(&final_input, &self.key, &self.mode)
    }

    fn encrypt(&self, prefix: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        let mut input = prefix.clone();
        input.extend(&self.data);
        aes::encrypt(&input, &self.key, &self.mode)
    }
}

macro_rules! strn {
    ( $c : expr, $n : expr ) => ( String::from_iter( ( 0 .. $n ).map( |_| $c as char).collect::<Vec<char>>() ) );
}

macro_rules! printr {
    ( $x : expr ) => ( try!(ascii::raw_to_str($x)) );
}


pub fn break_aes_ecb(cipherbox: &CipherBox) -> Result<String, err::Error> {
    let (blocksize, plaintext_size) = try!(detect_block_size(&cipherbox, max_blocksize));

    ctry!(!try!(oracle::detect_aes_ecb(&try!(cipherbox.gen(&strn!('A', 2 * blocksize))), blocksize)),
        "cipher is not aes ecb, can't break with this module");

    let max_u8 = 126;

    let mut plainraw = Vec::<u8>::new();
    let mut block_no: usize = 0;

    let mut prefix = vec![65 as u8; blocksize - 1];
    let mut dict_prefix = prefix.clone();

    for i in 0 .. plaintext_size {
        //println!("{} - {}", printr!(&prefix), printr!(&dict_prefix));

        let cipher = try!(cipherbox.encrypt(&prefix));
        let dict = try!(make_dict(&dict_prefix, &cipherbox, max_u8));

        let cipher_block: Vec<u8> = cipher.chunks(blocksize).nth(block_no).unwrap().to_vec();

        let mut raw_char: u8 = 0;

        for j in 0 .. dict.len() {
            if dict[j] == cipher_block {
                raw_char = j as u8 + 1;
                print!("{}", raw_char as char);
                io::stdout().flush();
            }
        }
        ctry!(raw_char == 0, format!("no match for character at pos: {} \n{}", i, try!(ascii::raw_to_str(&plainraw))));

        plainraw.push(raw_char);
        prefix.pop();
    
        dict_prefix = try!(shift_left_and_push(&dict_prefix, raw_char));

        if (i + 1) % blocksize == 0 {
            prefix = vec![65 as u8; blocksize - 1]; 
            block_no += 1;
        }
    }
    let plaintext = try!(ascii::raw_to_str(&plainraw));
    Ok(plaintext)
}


fn shift_left_and_push(input: &Vec<u8>, c: u8) -> Result<Vec<u8>, err::Error> {
    let mut input_iter = input.iter();
    input_iter.next();
    let mut result: Vec<u8> = input_iter.cloned().collect();
    result.push(c);
    Ok(result)
}


pub fn make_dict(prefix: &Vec<u8>, cipherbox: &CipherBox, max_u8: u8) -> Result<Vec<Vec<u8>>, err::Error> {
    let mut dict = Vec::<Vec<u8>>::new();
    let mut block = prefix.clone();

    for i in 1 .. max_u8 + 1 {
        block.push(i as u8);
        let cipher = try!(cipherbox.encrypt(&block));
        let cipher_block0 = cipher.chunks(block.len()).next().unwrap().to_vec();

        dict.push(cipher_block0);
        block.pop();
    }
    Ok(dict)
}


pub fn detect_block_size(cipherbox: &CipherBox, max: usize) -> Result<(usize, usize), err::Error> {
    let len1 = try!(cipherbox.encrypt(&Vec::<u8>::new())).len();

    let mut prefix = vec![65 as u8];
    for i in 0 .. max {
        let len2 = try!(cipherbox.encrypt(&prefix)).len();
        if len2 > len1 {
            return Ok((len2 - len1, len1 - prefix.len() + 1));
        }
        prefix.push(65 as u8);
    }
    mkerr!("failed to detect cipher block size")
}


pub fn init_cipherbox_from_file(filepath: &str) -> Result<CipherBox, err::Error> {
    let plain_base64 = try!(util::read_file_to_str(&filepath));
    let clean_base64 = try!(ascii::filter_whitespace(&plain_base64));

    init_cipherbox(&clean_base64)
}


pub fn init_cipherbox(plaintext_base64: &str) -> Result<CipherBox, err::Error> {
    let plainraw = try!(base64::base64_to_raw(&plaintext_base64));
    CipherBox::new(&plainraw, aes::ecb_128_pkcs7)
}


pub fn interactive() -> u32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data (base64 encoded) filepath"); return 1; }
    };

    let cipherbox = rtry!(init_cipherbox(&input_filepath), 1);
    let plaintext = rtry!(break_aes_ecb(&cipherbox), 1);

    println!("{}", plaintext);
    0
}

