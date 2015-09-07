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


const max_blocksize: usize = 32;
const max_random_data_length: usize = 512;


pub fn break_aes_ecb(cbox: &cb::CipherBox) -> Result<String, err::Error> {
    let blocksize = 16;

    let mut input = vec![65; blocksize * 3 - 1];
    let cipher = try!(cbox.encrypt(&input));
    let blockA = try!(find_cons_same_cipher_block(&cipher, blocksize));

    input = vec![65; blocksize * 2 - 1];

    let mut ciphers: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
    
    for _ in 0 .. blocksize * 5 {
        let cipher = try!(cbox.encrypt(&input));
        let mut block_iter = cipher.chunks(blocksize); 

        let mut b = block_iter.next();
        while b != None {
            if b.unwrap().to_vec() == blockA {
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

        ctry!(after_blockA.len() < blocksize * 2, "something is wrong");

        push_if_not_in(&mut ciphers, &after_blockA);
    }
    println!("ciphers len: {}", ciphers.len());
    ctry!(ciphers.len() != blocksize, "not all shifts in target data captured");
    
    let mut ord_ciphers = Vec::<Vec<u8>>::new();
    let mut plaintext = String::new();
    let mut prefix = vec![65; blocksize - 1];

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
                ord_ciphers.push(cipher.clone());
                ciphers.remove(j);
                prefix = try!(util::shift_left_and_push(&prefix, c as u8));
                break;
            }
        }
    }

    ctry!(ciphers.len() > 1, "only one cipher (0-shifted) should be left by now");
    ciphers.extend(ord_ciphers);
    ord_ciphers = ciphers;

    let len1 = ord_ciphers[0].len();
    let pos = ord_ciphers.iter().position(|v| v.len() != len1).unwrap();
    let rem_plaintext_len = ord_ciphers[pos].len() - (ord_ciphers.len() - 1 - pos) - plaintext.len();

    let mut ord_ciphers_it = ord_ciphers.iter().cycle();
    let mut block_no: usize = 0;

    for i in blocksize - 1 .. rem_plaintext_len {
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
    if vecs.iter().find(|&v| v == nvec)  == None {
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

