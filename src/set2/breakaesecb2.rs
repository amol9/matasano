use std::env;
use std::io;
use std::io::Write;


use common::{err, util, challenge};
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         14,
    title:      "Byte-at-a-time ECB decryption (Harder)",
    help:       "param1: path to base 64 encoded plain text file (to be used as target data)",
    execute_fn: interactive
};


const max_blocksize: usize = 32;
const max_random_data_length = 512;


pub fn break_aes_ecb(cbox: &cb::CipherBox) -> Result<String, err::Error> {
    let blocksize = 16;

    let mut input = vec![65; blocksize * 3 - 1];
    let cipher = try!(cbox.encrypt(&input));
    let blockA = try!(find_cons_same_cipher_block(&cipher));

    input = vec![65; blocksize * 2 - 1];

    let mut ciphers: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
    
    for _ in 0 .. blocksize * 3 {
        let cipher = try!(cbox.encrypt(&input));
        let mut block_iter = cipher.chunks(blocksize); 

        let b = block_iter.next();
        while b != None {
            if b == blockA {
                break;
            }
            b = block_iter.next();
        }

        let mut after_blockA = Vec<u8>::new();
        b = block_iter.next();

        while b != None {
            after_blockA.push(b);
            b = block_iter.next();
        }

        ctry!(after_blockA.len() < blocksize * 2, "something is wrong");

        push_if_not_in(&mut ciphers, &after_blockA);
    }
    ctry!(ciphers.len() != blocksize, "not all shifts in target data catpured");
    
    let mut ord_ciphers = Vec::<Vec<u8>>::new();
    let mut plaintext = String::new();

    for i in (blocksize - 1 .. 0).step_by(-1) {
        let dict = try!(cb::make_dict_for_random_prefox_cb(&vec![65; i], &cbox, &ascii::valid_chars, &blockA));
        
        for j in 0 .. ciphers.len() {
            let block0 = ciphers[j].chunks(blocksize).next().unwrap().to_vec();

            let pos = dict.position(|&b| b == block0);
            if pos != None {
                plaintext.push(ascii::valid_chars[pos]);
                ord_ciphers.push(cipher.clone());
                ciphers.remove(j);
                break;
            }
        }
    }

    ctry!(ciphers.len() > 1, "only one cipher (0-shifted) should be left by now");
    ord_ciphers.push(ciphers[0].clone());

    let len1 = ord_ciphers[0].len();
    let pos = ord_ciphers.position(|&v| v.len() != len1);
    let rem_plaintext_len = ord_ciphers[pos].len() - (ord_ciphers.len() - 1 - pos) - plaintext.len();

    for i in plaintext.len() .. rem_plaintext_len {
        
    }

    
}


fn push_if_not_in(vecs: &mut Vec<Vec<u8>>, nvec: &Vec<u8>) {
    match vecs.find(|&v| v == nvec) {
        Some(v) => {},
        None    => vecs.push(nvec)
    }
}


fn find_cons_same_cipher_block(cipher: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    let mut block_iter = cipher.chunks(blocksize);
    let mut b1 = block_iter.next().unwrap();

    for b2 in block_iter {
        if b1 == b2 {
            return Ok(b1.clone())
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
