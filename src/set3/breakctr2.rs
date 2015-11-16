use std::env;
use std::slice;
use std::io::prelude::*;

use common::{err, challenge};
use common::cipher::one_byte_xor as obx;
use set3::breakctr;


pub static info: challenge::Info = challenge::Info {
    no:         20,
    title:      "Break fixed-nonce CTR statistically",
    help:       "param1: path to file containing base64 encoded plain strings",
    execute_fn: interactive
};

pub const min_chars_for_charfreq: usize = 10;       // minimum number of characters required for breaking based on character frequency

// break ctr cipher one column at a time
// input:  a list of cipher strings encrypted usinf CTR with same nonce
// output: keystream
//
pub fn break_ctr(ciphers: &Vec<Vec<u8>>) -> Result<Vec<u8>, err::Error> {
    let mut cipher_its: Vec<slice::Iter<u8>> = Vec::new();
    let mut keystream = Vec::<u8>::new();

    for c in ciphers {
        cipher_its.push(c.iter());
    }

    let mut all_ciphers_done = false;
    let mut col_no = 0;

    while ! all_ciphers_done {
        //println!("col no: {}", col_no);

        let mut col = Vec::<u8>::new();         // extract a column
        for it in cipher_its.iter_mut() {
            match it.next() {
                Some(v) => col.push(*v),
                None    => {}
            };
        }

        let mut options = obx::GuessOptions::new();

        if col_no == 0 {        // first column has a lot of upper case letters, hence, does not break correctly
                                // with standard character frequencies
            fn upper_letters_distance_fn(input: &str) -> Result<f32, err::Error> {
                let count: usize = input.chars().filter(|c| *c >= 'A' && *c <= 'Z').count();
                Ok(1f32/ count as f32)      // inverse, since the key is guessed for minimum distance
            }

            options.set_distance_fn(upper_letters_distance_fn);
        }

        if col.len() < min_chars_for_charfreq && col.len() > 0 {        // the input is too short, so, we take a shot
                                                                        // assuming all are valid
                                                                        // word characters
            fn last_chars_distance_fn(input: &str) -> Result<f32, err::Error> {
                let count: usize = input.chars().filter(|c| (*c >= 'A' && *c <= 'Z') || (*c >= 'a' && *c <= 'z')).count();
                Ok(1f32 / count as f32)
            }

            options.set_distance_fn(last_chars_distance_fn)
        }
        
        if col.len() > 0 {
            keystream.push(try!(obx::guess_key(&col, Some(&options))).key);
        } else {
            all_ciphers_done = true;
        }
        col_no += 1;
    }

    Ok(keystream)
}

pub fn break_ctr_with_manual_guess_for_last_chars(ciphers: &Vec<Vec<u8>>, guesses: &Vec<(usize, &str)>) ->
    Result<Vec<String>, err::Error> {

    let keystream = try!(break_ctr(&ciphers));
    let plains = try!(breakctr::manual_guess_for_last_chars(&ciphers, &keystream, &guesses));
    Ok(plains)
}

pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input plain data (base64 encoded) filepath"); return exit_err!(); }
    };

    let ciphers = rtry!(breakctr::generate_ciphers_from_file(&input_filepath), exit_err!());
    let plains = rtry!(break_ctr_with_manual_guess_for_last_chars(&ciphers, &vec![]), exit_err!());

    for p in plains {
        println!("{}", p);
    }
    exit_ok!()
}

