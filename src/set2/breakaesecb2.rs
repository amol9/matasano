use std::env;
use std::io;
use std::io::Write;

use common::{err, util, challenge, ascii};
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         14,
    title:      "Byte-at-a-time ECB decryption (Harder)",
    help:       "param1: path to base 64 encoded plain text file (to be used as target data)",
    execute_fn: interactive
};


pub const max_random_data_length: usize = 512;
const special_input_char: char = 'A';


pub fn break_aes_ecb(cbox: &cb::CipherBox) -> Result<String, err::Error> {
    let blocksize = 16;

    let mut input = vec![special_input_char as u8; blocksize * 3 - 1];
    let cipher = try!(cbox.encrypt(&input));
    let blockA = try!(find_cons_same_cipher_block(&cipher, blocksize));

    let mut ciphers = try!(find_target_ciphers_for_each_byte_shift(&cbox, blocksize, &blockA));

    let (mut plaintext, ord_ciphers, finished) = try!(break_first_block(&cbox, blocksize, &blockA, &mut ciphers));

    if ! finished {
        plaintext = try!(break_rem_blocks(&cbox, blocksize, &blockA, &ord_ciphers, &plaintext));
    }

    println!("");
    Ok(plaintext)
}


// find target data ciphers for:
// prefix: (blocksize - 1) A's + target data
// prefix: (blocksize - 2) A's + target data
// prefix: (blocksize - 3) A's + target data
// to
// prefix: no prefix + target data
// total samples = blocksize
//
pub fn find_target_ciphers_for_each_byte_shift(cbox: &cb::CipherBox, blocksize: usize, blockA: &Vec<u8>) ->
    Result<Vec<Vec<u8>>, err::Error> {

    let input = vec![special_input_char as u8; blocksize * 2 - 1];

    let mut ciphers: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
    
    for _ in 0 .. blocksize * 7 {                               // we're at rand's mercy here
        let cipher = try!(cbox.encrypt(&input));                // so, we do large enough number of trials
        let mut block_iter = cipher.chunks(blocksize);          // to get target data ciphers of each byte shift    
    
        let mut b = block_iter.next();
        while b != None {
            if &b.unwrap().to_vec() == blockA {
                break;
            }
            b = block_iter.next();
        }

        let mut after_blockA = Vec::<u8>::new();
        b = block_iter.next();

        while b != None {
            after_blockA.extend(b.unwrap());
            b = block_iter.next();
        }

        ctry!(after_blockA.len() < blocksize, "not a valid cipher block after our special input");

        push_if_not_in(&mut ciphers, &after_blockA);
        if ciphers.len() == blocksize {
            break;
        }
    }

    ctry!(ciphers.len() != blocksize, "not all shifts in target data captured, a retry may work");
    Ok(ciphers)
}


// actually breaks the first blocksize - 1 characters
// but, the function name is chosen to be simple
// takes the shifted ciphers
// and does a dictionary attack on the first block
//
// also, orders them as following:
//  first item: target data cipher with 0-shift
//  the rest: in the decreasing order of right shift starting from
//   blocksize - 1 going to 1
// (the incoming shifted ciphers are not in order)
// 
// the cipher with 0-shift is chosen to be first since,
// that's the one we need first in our next step
//
// returns: plaintext (blocksize - 1 chars), ordered ciphers
//
fn break_first_block<'a>(cbox: &cb::CipherBox, blocksize: usize, blockA: &Vec<u8>, ciphers: &'a mut Vec<Vec<u8>>) ->
    Result<(String, &'a Vec<Vec<u8>>, bool), err::Error> {

    let mut ord_ciphers = Vec::<Vec<u8>>::new();
    let mut plaintext = String::new();
    let mut prefix = vec![special_input_char as u8; blocksize - 1];

    let valid_chars = ascii::valid_chars();

    for i in 0 .. (blocksize - 1) {
        let dict = try!(cb::make_dict_for_random_prefix_cb(&prefix, &cbox, &valid_chars, &blockA));
        
        for j in 0 .. ciphers.len() {
            let block0 = ciphers[j].chunks(blocksize).next().unwrap().to_vec();

            let pos = dict.iter().position(|b| *b == block0);

            if pos != None {
                let c = ascii::u8_to_char(valid_chars[pos.unwrap()]);
                printc!(c);

                plaintext.push(c);
                ord_ciphers.push(ciphers[j].clone());
                ciphers.remove(j);
                prefix = try!(util::shift_left_and_push(&prefix, c as u8));
                break;
            }
        }
    }

    let mut finished: bool = ciphers.len() > 1;         //plain text smaller than a blocksize
    ciphers.extend(ord_ciphers);

    Ok((plaintext, ciphers, finished))
}


// break the remaining cipher, i.e. blocksize - 1 to end
// in: ordered ciphers from previous step
// in: plaintext decrypted so far (blocksize - 1 chars)
//
// performs the usual aes ecb attack, char at a time
// returns: plaintext
//
fn break_rem_blocks(cbox: &cb::CipherBox, blocksize: usize, blockA: &Vec<u8>, ord_ciphers: &Vec<Vec<u8>>, partial_plaintext: &str) ->
    Result<String, err::Error> {

    ctry!(ord_ciphers.len() != blocksize, "ordered ciphers != block size");
    let mut plaintext = String::from(partial_plaintext);

    let len1 = ord_ciphers[0].len();                            // determine the length of remaining plain text
    let rem_plaintext_len = match ord_ciphers.iter().rev().position(|c| c.len() != len1) {
        Some(v) => ord_ciphers[blocksize - v - 1].len() - (v + 1) - (blocksize - 1) - plaintext.len(),
        None    => len1 - (blocksize - 1) - plaintext.len()     // all ciphers same size => plain text size = cipher size - (blocksize - 1)
    };

    let mut ord_ciphers_it = ord_ciphers.iter().cycle();
    let mut block_no: usize = 0;

    let mut prefix = raw!(&plaintext);
    let valid_chars = ascii::valid_chars();

    for i in blocksize - 1 .. rem_plaintext_len + blocksize - 1 {
        let dict = try!(cb::make_dict_for_random_prefix_cb(&prefix, &cbox, &valid_chars, &blockA));
        
        let cipher_block = ord_ciphers_it.next().unwrap().chunks(blocksize).nth(block_no).unwrap().to_vec();
        
        let c: char = match dict.iter().position(|v| *v == cipher_block) {
            Some(v) => ascii::u8_to_char(valid_chars[v]),
            None    => return mkerr!(format!("error at position: {}, cannot find matching block in dictionary", i))
        };

        printc!(c);
        plaintext.push(c as char);
        prefix = try!(util::shift_left_and_push(&prefix, c as u8));

        if (i + 1) % blocksize == 0 {
            block_no += 1;
        }
    }

    Ok(plaintext)    
}


fn push_if_not_in(vecs: &mut Vec<Vec<u8>>, nvec: &Vec<u8>) {
    if vecs.iter().find(|&v| v == nvec) == None {
        vecs.push(nvec.clone());
    }
}


fn find_cons_same_cipher_block(cipher: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    let mut block_iter = cipher.chunks(blocksize);
    let mut b1 = block_iter.next().unwrap();

    for b2 in block_iter {
        if b1 == b2 {
            return Ok(b1.to_vec())
        }
        b1 = b2;
    }
    mkerr!("cannot find two consecutive blocks of same cipher")
}


fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data (base64 encoded) filepath"); return exit_err!(); }
    };

    let mut cbox = rtry!(cb::init_from_file(&input_filepath), exit_err!());
    cbox.enable_random_prefix(max_random_data_length);

    let plaintext = rtry!(break_aes_ecb(&cbox), exit_err!());

    //println!("{}", plaintext);
    exit_ok!()
}

