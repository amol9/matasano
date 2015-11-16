use std::env;
use std::io;
use std::io::Write;

use common::{err, ascii, util, challenge};
use common::cipher::oracle;
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         12,
    title:      "Byte-at-a-time ECB decryption (Simple)",
    help:       "param1: path to base 64 encoded plain text file",
    execute_fn: interactive
};


const max_blocksize: usize = 32;


macro_rules! printr {
    ( $x : expr ) => ( try!(ascii::raw_to_str($x)) );
}


pub fn break_aes_ecb(cbox: &cb::CipherBox) -> Result<String, err::Error> {
    let (blocksize, plaintext_size) = try!(detect_blocksize_plainsize(&cbox, max_blocksize));

    ctry!(!try!(oracle::detect_aes_ecb(&try!(cbox.encrypt(&vec![65 as u8; 2 * blocksize])), blocksize)),
        "cipher is not aes ecb, can't break with this module");

    let max_u8 = 126;

    let mut plainraw = Vec::<u8>::new();
    let mut block_no: usize = 0;

    let mut prefix = vec![65 as u8; blocksize - 1];
    let mut dict_prefix = prefix.clone();

    for i in 0 .. plaintext_size {
        //println!("{} - {}", printr!(&prefix), printr!(&dict_prefix));

        let cipher = try!(cbox.encrypt(&prefix));
        let dict = try!(cb::make_dict(&dict_prefix, &cbox, max_u8));

        let cipher_block: Vec<u8> = cipher.chunks(blocksize).nth(block_no).unwrap().to_vec();

        let mut raw_char: u8 = 0;

        for j in 0 .. dict.len() {
            if dict[j] == cipher_block {
                raw_char = j as u8 + 1;
                printc!(raw_char as char);
            }
        }
        ctry!(raw_char == 0, format!("no match for character at pos: {} \n{}", i, try!(ascii::raw_to_str(&plainraw))));

        plainraw.push(raw_char);
        prefix.pop();
    
        dict_prefix = try!(util::shift_left_and_push(&dict_prefix, raw_char));

        if (i + 1) % blocksize == 0 {
            prefix = vec![65 as u8; blocksize - 1]; 
            block_no += 1;
        }
    }
    let plaintext = try!(ascii::raw_to_str(&plainraw));
    Ok(plaintext)
}


pub fn detect_blocksize_plainsize(cbox: &cb::CipherBox, max: usize) -> Result<(usize, usize), err::Error> {
    let len1 = try!(cbox.encrypt(&Vec::<u8>::new())).len();

    let mut prefix = vec![65 as u8];
    for _ in 0 .. max {
        let len2 = try!(cbox.encrypt(&prefix)).len();
        if len2 > len1 {
            return Ok((len2 - len1, len1 - prefix.len() + 1));
        }
        prefix.push(65 as u8);
    }
    mkerr!("failed to detect cipher block size")
}


pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data (base64 encoded) filepath"); return exit_err!(); }
    };

    let cbox = rtry!(cb::init_from_file(&input_filepath), exit_err!());
    rtry!(break_aes_ecb(&cbox), exit_err!());

    exit_ok!()
}

