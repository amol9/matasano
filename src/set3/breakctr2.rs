use std::env;
use std::slice;
use std::io;
use std::io::prelude::*;

use common::{err, challenge, ascii, base64, util, charfreq};
use common::cipher::one_byte_xor as obx;
use common::cipher::aes;
use set3::breakctr;


pub static info: challenge::Info = challenge::Info {
    no:         20,
    title:      "Break fixed-nonce CTR statistically",
    help:       "param1: path to file containing base64 encoded plain strings",
    execute_fn: interactive
};

// break ctr cipher one column at a time
// input:  a list of cipher strings encrypted usinf CTR with same nonce
// output: a corresponding list of decrypted plain texts and keystream
//
pub fn break_ctr(ciphers: &Vec<Vec<u8>>) -> Result<(Vec<String>, Vec<u8>), err::Error> {
    let mut cipher_its: Vec<slice::Iter<u8>> = Vec::new();
    let mut keystream = Vec::<u8>::new();

    for c in ciphers {
        cipher_its.push(c.iter());
    }

    let mut all_ciphers_done = false;
    let mut col_no = 0;

    while ! all_ciphers_done {
        //println!("col no: {}", col_no);
        col_no += 1;

        let mut col = Vec::<u8>::new();         // extract a column
        for it in cipher_its.iter_mut() {
            match it.next() {
                Some(v) => col.push(*v),
                None    => {}
            };
        }

        if col.len() > 0 {
            keystream.push(try!(obx::guess_key(&col, None)).key);
        } else {
            all_ciphers_done = true;
        }
    }

    Ok((breakctr::xor_keystream(&ciphers, &keystream), keystream))
}

pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input plain data (base64 encoded) filepath"); return exit_err!(); }
    };

    let ciphers = rtry!(breakctr::generate_ciphers_from_file(&input_filepath), exit_err!());
    let (plains, _) = rtry!(break_ctr(&ciphers), exit_err!());

    for p in plains {
        println!("{}", p);
    }
    exit_ok!()
}

