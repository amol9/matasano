use std::env;
use std::iter::FromIterator;

use common::{err, ascii, base64, util};
use common::cipher::{aes, oracle, key};


const max_blocksize: usize = 32;


pub struct CipherBox {
    key:    Vec<u8>,
    data:   Vec<u8>,
    mode:   aes::Mode
}


impl CipherBox {
    fn new(data: &str, mode: aes::Mode) -> Result<Self, err::Error> {
        Ok(CipherBox {
            key:    try!(key::random(mode.blocksize)),
            data:   try!(ascii::str_to_raw(&data)),
            mode:   mode
        })
    }

    fn gen(&self, prefix: &str) -> Result<Vec<u8>, err::Error> {
        let mut final_input = try!(ascii::str_to_raw(&prefix));
        final_input.extend(&self.data);

        aes::encrypt(&final_input, &self.key, &self.mode)
    }
}

macro_rules! strn {
    ( $c : expr, $n : expr ) => ( String::from_iter( ( 0 .. $n ).map( |_| $c as char).collect::<Vec<char>>() ) );
}


pub fn break_aes_ecb(cipherbox: &CipherBox) -> Result<String, err::Error> {
    let blocksize = try!(detect_block_size(&cipherbox, max_blocksize));
    println!("blocksize = {}", blocksize);

    ctry!(!try!(oracle::detect_aes_ecb(&try!(cipherbox.gen(&strn!('A', 2 * blocksize))), blocksize)),
        "cipher is not aes ecb, can't break with this module");

    let max_u8 = 126;

    let mut plaintext = String::new();
    let mut block_no: usize = 0;

    let mut prefix = strn!('A', blocksize - 1);
    let mut dec_prefix = String::new();
    let mut dict_prefix = prefix.clone();

    for i in 0 .. try!(cipherbox.gen("")).len() {
        println!("{} - {}", prefix, dict_prefix);
        let cipher = try!(cipherbox.gen(&prefix));
        let dict = try!(make_dict(&dict_prefix, &cipherbox, max_u8));

        let cipher_block: Vec<u8> = cipher.chunks(blocksize).nth(block_no).unwrap().to_vec();
        //println!("cipher block len: {}", cipher_block.len());

        let mut dec_char: Option<char> = None;
        for j in 0 .. dict.len() {
            if dict[j] == cipher_block {
                dec_char = Some((j as u8 + 1 as u8) as char);
                println!("match char: {}", dec_char.unwrap() as u8);
            }
        }
        ctry!(dec_char == None, format!("no match for character at pos: {} \n{}", i, plaintext));

        plaintext.push(dec_char.unwrap());
        prefix.pop();
    
        //shift left and insert the new char at end
        dict_prefix = try!(shift_left_and_insert(&dict_prefix, dec_char.unwrap()));

        if (i + 1) % blocksize == 0 {
            prefix = strn!('A', blocksize - 1);
            block_no += 1;
        }
    }
    Ok(plaintext)
}


fn shift_left_and_insert(input: &str, c: char) -> Result<String, err::Error> {
    let mut input_chars = input.chars();
    input_chars.next();
    let mut new = String::from_iter(input_chars.map(|c| c).collect::<Vec<char>>());
    new.push(c);
    Ok(new)
}


pub fn make_dict(prefix: &str, cipherbox: &CipherBox, max_u8: u8) -> Result<Vec<Vec<u8>>, err::Error> {
    let mut dict = Vec::<Vec<u8>>::new();
    let mut plain = String::from(prefix);

    for i in 1 .. max_u8 + 1 {
        plain.push(i as char);
        let cipher = try!(cipherbox.gen(&plain));
        let cipher_block0 = cipher.chunks(plain.len()).next().unwrap().to_vec();
        //println!("cipher block 0 len: {}", plain);
        dict.push(cipher_block0);
        plain.pop();
    }
    Ok(dict)
}


pub fn detect_block_size(cipherbox: &CipherBox, max: usize) -> Result<usize, err::Error> {
    let len1 = try!(cipherbox.gen("")).len();

    let mut prefix = String::from("A");
    for i in 0 .. max {
        let len2 = try!(cipherbox.gen(&prefix)).len();
        if len2 > len1 {
            return Ok(len2 - len1);
        }
        prefix.push('A');
    }
    mkerr!("failed to detect cipher block size")
}


pub fn init_cipherbox(filepath: &str) -> Result<CipherBox, err::Error> {
    let plainbase64 = try!(util::read_file_to_str(&filepath));
    let plainclean = try!(ascii::filter_whitespace(&plainbase64));
    let plainraw = try!(base64::base64_to_raw(&plainclean));
    let plaintext = try!(ascii::raw_to_str(&plainraw));

    //println!("{}", plaintext);
    CipherBox::new(&plaintext, aes::ecb_128_pkcs7)
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
