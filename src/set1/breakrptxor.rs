use std::env;
use std::ops;
use std::slice;
use std::clone;

use common::{err, util, ascii, base64, charfreq, challenge};
use common::cipher::one_byte_xor as obx;
use common::cipher::rpt_key_xor as rkx;


pub static info: challenge::Info = challenge::Info {
    no:         6,
    title:      "Break repeating-key XOR",
    help:       "param1: path to file containing repeating key xor encrypted data in base64 form",
    execute_fn: interactive
};


const KEYLENGTH_RANGE: ops::Range<usize> = ops::Range {start: 1, end: 40};
const BLOCK_COUNT: usize = 4;
const KEYLENGTH_GUESS_COUNT: usize = 4;


pub struct Guess {
    pub key:        String,
    pub distance:   f32,
    pub text:       String
}


impl clone::Clone for Guess {
    fn clone(&self) -> Self {
        Guess {
            key:        self.key.clone(),
            distance:   self.distance,
            text:       self.text.clone()
        }
    }
}


pub fn break_cipher_from_file(filepath: &str) -> Result<Guess, err::Error> {
    let cipherbase64 = try!(util::read_file_to_str(&filepath));
    let cipherclean = try!(ascii::filter_whitespace(&cipherbase64));
    let cipherraw = try!(base64::base64_to_raw(&cipherclean));

    break_cipher(&cipherraw)
}


pub fn break_cipher(cipherraw: &Vec<u8>) -> Result<Guess, err::Error> {
    let keylengths = try!(guess_key_length(&cipherraw, KEYLENGTH_GUESS_COUNT));
    let mut guesses: Vec<Guess> = Vec::new();

    for keylength in keylengths {
        let key = try!(guess_key(&cipherraw, keylength));

        let guessraw = try!(rkx::decrypt_raw(&cipherraw, &key));
        let guesstext = try!(ascii::raw_to_str(&guessraw));

        let dist: f32 = try!(charfreq::distance_from_base(&guesstext));

        let keystr = try!(ascii::raw_to_str(&key));

        guesses.push(Guess {
            key:        keystr,
            distance:   dist,
            text:       guesstext
        });
    }

    let best_guess = guesses[util::min_index(&guesses.iter().map(|g| g.distance).collect()).unwrap()].clone();

    Ok(best_guess)
}


pub fn guess_key_length(cipherraw: &Vec<u8>, guess_count: usize) -> Result<Vec<usize>, err::Error> {
    let mut dist: Vec<f32> = Vec::new();

    let mut krange = KEYLENGTH_RANGE;
    if cipherraw.len() / 2 < krange.end{
        krange.end = cipherraw.len() / 2
    }

    for block_len in krange {
        let mut blocks: slice::Chunks<u8> = cipherraw.chunks(block_len);
        let mut d_avg: Option<f32> = None;

        let mut b1 = match blocks.next() {
            Some(v) => v.to_vec(),
            None    => break
        };

        let mut bc: usize = 1;
        for b2 in blocks {
            if bc == BLOCK_COUNT {
                break;
            }

            if b1.len() != b2.len() {
                break;
            }

            let d = etry!(block_hamm(&b1, &b2.to_vec()), "hamming distance calculation error");
            d_avg = match d_avg {
                Some(v) => Some((v + d as f32) / 2 as f32),
                None    => Some(d as f32)
            };

            b1 = b2.to_vec();
            bc += 1;
        }
        dist.push(d_avg.unwrap() as f32 / block_len as f32);
        d_avg = None;
    }
    //for i in 0 .. dist.len() {
    //    println!("{}: {}", i, dist[i]);
    //}
    
    let kl: Vec<usize> = util::min_indices(&dist, guess_count).unwrap();
    let keylengths = kl.iter().map(|l| *l as usize + KEYLENGTH_RANGE.start).collect();

    Ok(keylengths)
}


fn block_hamm(b1: &Vec<u8>, b2: &Vec<u8>) -> Result<u32, err::Error> {
        //Ok(util::hamming_distance(b1[0], b2[0]) as u32)
        util::hamm_vec(&b1, &b2)
}

pub fn guess_key(cipherraw: &Vec<u8>, keylength: usize) -> Result<Vec<u8>, err::Error> {
    let cipher_trans = etry!(util::transpose_vec(&cipherraw, keylength as u32), "transpose error");

    let mut key = Vec::new();
    for slice in cipher_trans {
        let keybyte: u8 = try!(obx::guess_key(&slice, None)).key;
        key.push(keybyte);
    }
    Ok(key)
}


pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data filepath"); return exit_err!(); }
    };

    let guess = rtry!(break_cipher_from_file(&input_filepath), 1);
    println!("\nkey: {}\n\n{}", guess.key, guess.text);
    exit_ok!()
}

