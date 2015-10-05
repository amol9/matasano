use std::env;
use std::slice;
use std::io;
use std::io::prelude::*;

use common::{err, challenge, ascii, base64, util, charfreq};
use common::cipher::one_byte_xor as obx;
use common::cipher::aes;
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         19,
    title:      "Break fixed-nonce CTR mode using substitions",
    help:       "param1: path to file containing base64 encoded plain strings",
    execute_fn: interactive
};


// heuristically determined constants:
const trigrams_limit: usize = 50;                   // number of trigrams to use for guessing using duplicate occurrences
const trigrams_limit_last_characters: usize = 400;  // number of trigrams to use for guessing last few characters
const trigrams_key_limit: usize = 20;               // number of candidate keys to use for guessing after sorting by (weight * count)
const trigrams_min_dups: usize = 3;                 // minimum number of trigram duplicates needed in a column
const min_prefixes_for_last_chars: usize = 3;       // minimum number of prefixes needed to guess the last few characters without weights


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

    let (tri_idx, tri_c) = detect_trigrams(&ciphers);

    let mut tri_idx_it = tri_idx.iter();        // iterator over trigram column indexes
    let mut tri_c_it = tri_c.iter();            // iterator over corresponding lists of cipher characters    

    let mut all_ciphers_done = false;
    let mut col_no = 0;

    while !all_ciphers_done {
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
            let tidx = match tri_idx_it.next() {        // trigram column index: 0, 1 or 2
                Some(v) => *v,                          // 3: if no trigram
                None    => 3
            };

            let empty_vec = Vec::<u8>::new();

            let tc = match tri_c_it.next() {            // cipher character list for the column
                Some(v) => v,                           // (those found as trigram duplicates)
                None    => &empty_vec
            };

            let cw: (Vec<u8>, Vec<u32>);
            if tidx < 3 && tc.len() > trigrams_min_dups {
                cw = try!(filter_candidates(tidx, &tc));
            } else {
                cw = try!(filter_candidates_for_last_chars(&ciphers, &keystream));
            }

            let weights: Vec<f32> = cw.1.iter().map(|v| *v as f32).collect();
            let mut options = obx::GuessOptions::new();
            if cw.0.len() > 0 {
                try!(options.set_candidates(&cw.0, &weights));
            }

            keystream.push(try!(obx::guess_key(&col, Some(&options))).key);
        } else {
            all_ciphers_done = true;
        }
    }

    Ok((xor_keystream(&ciphers, &keystream), keystream))
}


fn filter_candidates(col: usize, c: &Vec<u8>) -> Result<(Vec<u8>, Vec<u32>), err::Error> {
    let tri_col: Vec<u8> = trigrams_col_no_weights!(col, trigrams_limit, "");
    let result: Vec<u8>;
    let weights: Vec<u32> = Vec::<u32>::new();      // will not contain weights

    //println!("dup count: {}", c.len());

    if c.len() == 1 {
        result = tri_col.iter().map(|tc| tc ^ c[0]).collect();
    } else {

        let mut r = Vec::<u8>::new();

        let cipher_chars_freq = util::freq(&c);
        let trigram_chars_freq = util::freq(&tri_col);

        for ccf in cipher_chars_freq.iter() {       // eliminate candidates
                                                    // e.g. if a cipher char's freq = 2, then only
                                                    // select trigram chars whose freq >= 2
            let filtered_by_freq = trigram_chars_freq.iter().filter(|tcf| tcf.1 >= ccf.1 && tcf.0 != ('#' as u8));
            let keys: Vec<u8> = filtered_by_freq.map(|fbf| fbf.0 ^ ccf.0).collect();
            r.extend(&keys);
        }

        let mut r2: Vec<(u8, usize)> = util::freq(&r);      // get a count of occurrence of each of the keys
        r2.sort_by(|&a, &b| (b.1).cmp(&(a.1)));             // sort by descending count
                                                            // only take the first "n" candidate keys
        result = (0 .. trigrams_key_limit).zip(&r2).map(|(_, &t)| (t as (u8, usize)).0).collect();
    }

    //println!("candidates: {}", rawd!(&result));
    Ok((result, weights))
}


// for the last few characters (columns for which the count of duplicate trigrams detected < 3),
// we use last 2 decrypted characters as a prefix to predict next character using the trigram list
//
fn filter_candidates_for_last_chars(ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>) -> Result<(Vec<u8>, Vec<u32>), err::Error> {
    let mut prefixes = Vec::<(Vec<u8>, u8, usize)>::new();         // all prefixes of length 2

    let mut ks_it = keystream.iter().rev();
    let k2 = ks_it.next().unwrap();
    let k1 = ks_it.next().unwrap();

    let ks_len = keystream.len();
    let vnl = ascii::valid_non_letters();

    let replace_non_letter_by_hash = |p| match vnl.iter().any(|&c| c == p) {            // so, d, becomes d#
                                            true  => '#' as u8,                         // this'll help lookup matching trigrams
                                            false => p };

    for cipher in ciphers {                                                 // for each of the ciphers, decrypt the last 2 bytes
        if cipher.len() >= ks_len + 1 {                                     // using the keystream generated so far
            let mut prefix = Vec::<u8>::new();                              // predict the next plain character using matching trigrams
                                                                            // with the same 2-byte prefix
            prefix.push(replace_non_letter_by_hash(cipher[ks_len - 2] ^ k1));
            prefix.push(replace_non_letter_by_hash(cipher[ks_len - 1] ^ k2));

            //println!("{}", rts!(&prefix));
            prefixes.push((prefix, cipher[ks_len], cipher.len() - ks_len));
        }
    }

    let mut r = Vec::<(u8, u32)>::new();        // hold the (key, weight) pairs before sorting
    let not_enough_prefixes = prefixes.len() < min_prefixes_for_last_chars;

    for p in prefixes {
        let tcol: Vec<(u8, u32)> = try!(charfreq::trigrams_col(2, trigrams_limit_last_characters, rts!(&p.0).as_ref()));

        let letter_keys: Vec<(u8, u32)> = tcol.iter().filter(|&u| u.0 != '#' as u8).map(|u| (u.0 ^ p.1, u.1)).collect();

        let non_letter_keys: Vec<(u8, u32)> = tcol.iter().filter(|&u| u.0 == '#' as u8).map(|u| (u.0 ^ p.1, u.1)).collect();

        r.extend(letter_keys);                  // add candidate keys for letters
        r.extend(non_letter_keys);              // add candidate keys for non-letters
    }

    let mut r2: Vec<((u8, u32), usize)> = util::freq(&r);   // get a count of occurrence of each of the keys
    r2.sort_by(|&a, &b| (b.1 as u32 * (b.0).1).cmp(&(a.1 as u32 * (a.0).1)));   // sort by descending (weight * count)

    //for i in r2.iter() {
    //    println!("({}, {}), {}", (i.0).0, (i.0).1, i.1);
    //}
                                                            // only take the first "n" candidate keys
    let (mut result, mut weights): (Vec<u8>, Vec<u32>) = (0 .. trigrams_key_limit).zip(r2).map(|(_, t)| ((t.0).0, (t.0).1)).unzip();
    if ! not_enough_prefixes {
        weights.clear();
    } 
    
    //println!("candidates: {}", rawd!(&result));
    Ok((result, weights))
}


// detect trigrams (by detecting repeating 3-byte patterns starting at a column)
// return:
// 1. a vector of size = max cipher length
//    with 0, 1 or 2 to indicate trigram character column in each position
//    3: no trigram detected in that position
// 2. a vector of size = max cipher length
//    with trigram bytes (cipher) if detected in that position
//    else: empty vector
//
pub fn detect_trigrams(ciphers: &Vec<Vec<u8>>) -> (Vec<usize>, Vec<Vec<u8>>) {
    let mut cipher_its: Vec<slice::Iter<u8>> = Vec::new();
    for c in ciphers {
        cipher_its.push(c.iter());
    }

    let mut result_i = Vec::<usize>::new();         // column number 0, 1 or 2 of the detected duplicate trigram
    let mut result_c = Vec::<Vec<u8>>::new();       // cipher characters for each of the duplicate trigram detected in that column

    let mut buf = Vec::<Vec<u8>>::new();            // a cycling 3-byte buffer for trigram duplicate detection
    for _ in 0 .. ciphers.len() {
        buf.push(vec![0; 3]);
    }

    let mut idx = 0;                                // cipher byte index
    let mut all_ciphers_done = false;

    while ! all_ciphers_done {
        {
            let mut buf_it = buf.iter_mut();
            all_ciphers_done = true;

            for it in cipher_its.iter_mut() {
                let c = match it.next() {
                    Some(v) => { all_ciphers_done = false; v },         // if all cipher iters yield none, we are done
                    None    => { let mut b = buf_it.next().unwrap(); b.clear(); continue; }
                };

                let t = buf_it.next().unwrap();
                t[0] = t[1]; t[1] = t[2]; t[2] = *c;        // cycle the buffer
            }
        }

        if all_ciphers_done {
            break;
        }

        //println!("-");
        result_i.push(3);
        result_c.push(vec![]);

        if idx >= 2 {
            let trigrams_left = buf.iter().filter(|t| t.len() != 0).count() > 1;

            if trigrams_left {
                let dups = util::dup::<Vec<u8>>(&buf);

                for dup in dups {
                    if dup.0.len() != 0 {
                        //println!("dup: {}, {}", ascii::raw_to_str(&dup.0).unwrap(), dup.1);
                        result_i[idx - 2] = 0; result_c[idx - 2].push(dup.0[0]);
                        result_i[idx - 1] = 1; result_c[idx - 1].push(dup.0[1]);
                        result_i[idx]     = 2; result_c[idx].push(dup.0[2]);
                    }
                }
            } else {
                all_ciphers_done = true;
            }
        }        
        idx += 1;
    }
    (result_i, result_c)
}


pub fn xor_keystream(ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>) -> Vec<String> {
    let mut result = Vec::<String>::new();

    for c in ciphers {
        result.push(c.iter().zip(keystream.iter()).map(|(&c, &k)| chr!(c ^ k)).collect());
    }
    result
}


pub fn break_column(col: &Vec<u8>, candidates: &Vec<u8>, weights: &Vec<u32>) -> Result<u8, err::Error> {
    ctry!(weights.len() > 0 && candidates.len() != weights.len(), "all candidates must have weight");

    let mut dist = Vec::<f32>::new();
    let mut keys: Vec<u8>;
    let no_weights = weights.len() == 0;
    let mut weights_it = weights.iter();

    if candidates.len() == 0 {                  // if no candidate keys are provided, use brute force
        keys = (0 .. 255).collect();
        keys.push(255);
    } else {
        keys = candidates.iter().cloned().collect();
    }

    for i in keys.iter() {
        let xcol = col.iter().map(|&u| u ^ i).collect();
        let d = try!(charfreq::distance_from_base(rts!(&xcol).as_ref()));
        if no_weights {
            dist.push(d);
        } else {
            dist.push(d / *weights_it.next().unwrap() as f32);
        }
    }

    let key = keys[util::min_index(&dist).unwrap()];
    //let s: String = col.iter().map(|&u| chr!(u ^ k as u8)).collect();          
    //println!("col: {}", s);
    
    Ok(key as u8)
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


// this function will keep asking the user for his guesses for last characters
// it first calls auto decryption, then expects the user to supply guesses
// a guess is provided as: line no, suffix
// e.g. > 4, head
// use # for wildcard
// e.g. > 4, hat# or > 4, ##er, etc.
// enter nothing to exit the loop
//
// input parameter guesses is intended for automated testing
//
pub fn break_ctr_with_manual_guess_for_last_chars(ciphers: &Vec<Vec<u8>>, guesses: &Vec<(usize, &str)>) ->
    Result<Vec<String>, err::Error> {

    let (mut plains, mut keystream) = try!(break_ctr(&ciphers));

    fn display(plains: &Vec<String>) {      // display plain text lines with line numbers
        let mut c = 0;
        for p in plains {
            println!("{:02} {}", c, p);
            c += 1;
        }
    }

    fn fix_keystream(line_no: usize, suffix: &str, ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        let suffix_r = raw!(&suffix);
        let cipher: &Vec<u8> = &ciphers[line_no];
        let new_ks_bytes: Vec<u8> = cipher.iter().rev().zip(suffix_r.iter().rev()).map(|(c, s)| c ^ s).rev().collect();

        let mut new_keystream = keystream.clone();
        let ks_start = cipher.len() - suffix_r.len();

        for i in 0 .. new_ks_bytes.len() {
            if suffix_r[i] != '#' as u8 {
                new_keystream[ks_start + i] = new_ks_bytes[i];
            }
        }
        Ok(new_keystream)
    };

    if guesses.len() > 0 {              // guesses provided, so, don't ask the user
        for guess in guesses {
            keystream = try!(fix_keystream(guess.0, guess.1, &ciphers, &mut keystream));
            plains = xor_keystream(&ciphers, &keystream);
        }
    } else {                            // interact with user
        display(&plains);

        while true {
            let user_input = try!(util::input("enter guess (line no, last chars) [blank to exit]: "));

            if user_input.trim() == "" {
                break;
            }

            let parts: Vec<&str> = user_input.splitn(2, ",").collect();
            ctry!(parts.len() != 2, "need two values: line number, suffix");

            let line_no = etry!(parts[0].parse::<usize>(), format!("{} is not a valid number", parts[0]));
            let suffix = parts[1].trim();

            keystream = try!(fix_keystream(line_no, &suffix, &ciphers, &mut keystream));
            plains = xor_keystream(&ciphers, &keystream);

            display(&plains);
        }
    }

    Ok(plains)
}


pub fn interactive() -> err::ExitCode {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input plain data (base64 encoded) filepath"); return exit_err!(); }
    };

    let ciphers = rtry!(generate_ciphers_from_file(&input_filepath), exit_err!());
    let plains = rtry!(break_ctr_with_manual_guess_for_last_chars(&ciphers, &vec![]), exit_err!());

    for p in plains {
        println!("{}", p);
    }
    exit_ok!()
}

