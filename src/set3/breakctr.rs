use std::env;
use std::slice;

use common::{err, challenge, ascii, base64, util, charfreq};
use common::cipher::aes;
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         19,
    title:      "Break fixed-nonce CTR mode using substitions",
    help:       "",
    execute_fn: interactive
};


pub fn break_ctr(ciphers: &Vec<Vec<u8>>) -> Result<Vec<String>, err::Error> {
    let mut cipher_its: Vec<slice::Iter<u8>> = Vec::new();
    let mut keystream = Vec::<u8>::new();

    for c in ciphers {
        cipher_its.push(c.iter());
    }

    let mut all_ciphers_done = false;
    while !all_ciphers_done {
        let mut col = Vec::<u8>::new();
        for it in cipher_its.iter_mut() {
            match it.next() {
                Some(v) => col.push(*v),
                None    => {}
            };
        }

        if col.len() > 0 {
            keystream.push(try!(break_column(&col)));
        } else {
            all_ciphers_done = true;
        }
    }

    Ok(xor_keystream(&ciphers, &keystream))
}


fn xor_keystream(ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>) -> Vec<String> {
    let mut result = Vec::<String>::new();

    for c in ciphers {
        result.push(c.iter().zip(keystream.iter()).map(|(&c, &k)| chr!(c ^ k)).collect());
    }
    result
}


fn break_column(col: &Vec<u8>) -> Result<u8, err::Error> {
    let mut dist = Vec::<f32>::new();

    for i in 0 .. 256 {
        let xcol = col.iter().map(|&u| u ^ i).collect();
        dist.push(try!(charfreq::distance_from_base(rts!(&xcol).as_ref())));
    }

    //let keys = util::min_indices(&dist, 30).unwrap();
    let key_scores: Vec<usize> = (0 .. 256).map(|k| col.iter().filter(|&u| (*u ^ k as u8) > 128).count()).collect();
    let k = util::min_index(&key_scores).unwrap();

    let s: String = col.iter().map(|&u| chr!(u ^ k as u8)).collect();
                
    println!("col: {}", s);
    Ok(k as u8)
}


pub fn generate_ciphers_from_file(filepath: &str) -> Result<Vec<Vec<u8>>, err::Error> {
    let text = try!(util::read_file_to_str(&filepath));

    let cbox = try!(cb::CipherBox::new(&vec![], aes::ctr_128));
    let mut ciphers = Vec::<Vec<u8>>::new();

    for line in text.lines() {
        ciphers.push(try!(cbox.encrypt(&b64d!(&line))));
    }
    Ok(ciphers)
}


pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input plain data (base64 encoded) filepath"); return exit_err!(); }
    };

    let ciphers = rtry!(generate_ciphers_from_file(&input_filepath), exit_err!());
    let plains = rtry!(break_ctr(&ciphers), exit_err!());

    for p in plains {
        println!("{}", p);
    }
    exit_ok!()
}

