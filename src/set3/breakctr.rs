use std::env;
use std::slice;
use std::clone;

use common::{err, challenge, ascii, base64, util, charfreq};
use common::cipher::aes;
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         19,
    title:      "Break fixed-nonce CTR mode using substitions",
    help:       "",
    execute_fn: interactive
};


const trigrams_limit: usize = 50;
const trigrams_key_limit: usize = 30;
const trigrams_min_dups: usize = 3;     // minimum number of trigram duplicates needed in a column


pub fn break_ctr(ciphers: &Vec<Vec<u8>>) -> Result<Vec<String>, err::Error> {
    let mut cipher_its: Vec<slice::Iter<u8>> = Vec::new();
    let mut keystream = Vec::<u8>::new();

    for c in ciphers {
        cipher_its.push(c.iter());
    }

    let (tri_idx, tri_c) = detect_trigrams(&ciphers);

    let mut tri_idx_it = tri_idx.iter();
    let mut tri_c_it = tri_c.iter();

    let mut all_ciphers_done = false;
    let mut col_no = 0;

    while !all_ciphers_done {
        println!("col no: {}", col_no);
        col_no += 1;

        let mut col = Vec::<u8>::new();
        for it in cipher_its.iter_mut() {
            match it.next() {
                Some(v) => col.push(*v),
                None    => {}
            };
        }

        if col.len() > 0 {
            let mut candidates = Vec::<u8>::new();
            let mut weights = Vec::<u32>::new();

            let tidx = match tri_idx_it.next() {
                Some(v) => *v,
                None    => 3
            };

            let empty_vec = Vec::<u8>::new();

            let tc = match tri_c_it.next() {
                Some(v) => v,
                None    => &empty_vec
            };

            if tidx < 3 && tc.len() > trigrams_min_dups {
                let cw = try!(narrow_candidates(tidx, &tc));
                candidates = cw.0;
                weights = cw.1;
            } else {
                let cw = try!(narrow_candidates_for_last_chars(&ciphers, &keystream));
                candidates = cw.0;
                weights = cw.1;
            }

            keystream.push(try!(break_column(&col, &candidates, &weights)));
        } else {
            all_ciphers_done = true;
        }
    }

    Ok(xor_keystream(&ciphers, &keystream))
}


fn narrow_candidates(col: usize, c: &Vec<u8>) -> Result<(Vec<u8>, Vec<u32>), err::Error> {
    let tri_col: Vec<u8> = trigrams_col_no_weights!(col, trigrams_limit, "");
    let result: Vec<u8>;
    let weights: Vec<u32> = Vec::<u32>::new();

    println!("dup count: {}", c.len());

    if c.len() == 1 {
        result = tri_col.iter().map(|tc| tc ^ c[0]).collect();
    } else {

        let mut r = Vec::<u8>::new();

        let cipher_chars_freq = util::freq(&c);
        let trigram_chars_freq = util::freq(&tri_col);

        for ccf in cipher_chars_freq.iter() {
            let filtered_by_freq = trigram_chars_freq.iter().filter(|tcf| tcf.1 >= ccf.1 && tcf.0 != ('#' as u8));
            let keys: Vec<u8> = filtered_by_freq.map(|fbf| fbf.0 ^ ccf.0).collect();
            r.extend(&keys);
        }

        result = (0 .. trigrams_key_limit).zip(util::freq(&r).iter()).map(|(_, &t)| (t as (u8, usize)).0).collect();
    }

    println!("candidates: {}", rawd!(&result));
    Ok((result, weights))
}


fn narrow_candidates_for_last_chars(ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>) -> Result<(Vec<u8>, Vec<u32>), err::Error> {
    let mut prefixes = Vec::<(Vec<u8>, u8)>::new();

    let mut ks_it = keystream.iter().rev();
    let k2 = ks_it.next().unwrap();
    let k1 = ks_it.next().unwrap();

    let ks_len = keystream.len();
    let vnl = ascii::valid_non_letters();

    for cipher in ciphers {
        if cipher.len() >= ks_len + 1 {
            let mut prefix = Vec::<u8>::new();
            
            let mut c1 = cipher[ks_len - 2] ^ k1;
            c1 = match vnl.iter().any(|&c| c == c1) {
                true  => '#' as u8,
                false => c1
            };
            prefix.push(c1);

            let mut c2 = cipher[ks_len - 1] ^ k2;
            c2 = match vnl.iter().any(|&c| c == c2) {
                true  => '#' as u8,
                false => c2
            };
            prefix.push(c2);

            println!("{}", rts!(&prefix));
            prefixes.push((prefix, cipher[ks_len]));
        }
    }

    let mut r = Vec::<(u8, u32)>::new();
    let add_weights = prefixes.len() < 4;

    for p in prefixes {
        let t: Vec<(u8, u32)> = try!(charfreq::trigrams_col(2, trigrams_limit, rts!(&p.0).as_ref()));
        let keys: Vec<(u8, u32)> = t.iter().filter(|&u| u.0 != '#' as u8).map(|u| (u.0 ^ p.1, u.1)).collect();

        let keys2: Vec<(u8, u32)> = match t.iter().find(|&c| c.0 == '#' as u8) {
            Some(v) => vnl.iter().map(|&c| (c ^ p.1, v.1)).collect(),
            None    => vec![]
        };

        r.extend(&keys);
        r.extend(&keys2);
        
    }

    let (result, weights) = (0 .. trigrams_key_limit + 20).zip(util::freq(&r).iter()).map(|(_, &t)| ((t.0).0, (t.0).1)).unzip();
    println!("candidates: {}", rawd!(&result));
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

    let mut result_i = Vec::<usize>::new();
    let mut result_c = Vec::<Vec<u8>>::new();

    let mut buf = Vec::<Vec<u8>>::new();
    for _ in 0 .. ciphers.len() {
        buf.push(vec![0; 3]);
    }

    let mut idx = 0;
    let mut all_ciphers_done = false;

    while ! all_ciphers_done {
        {
            let mut buf_it = buf.iter_mut();
            all_ciphers_done = true;

            for it in cipher_its.iter_mut() {
                let c = match it.next() {
                    Some(v) => { all_ciphers_done = false; v },
                    None    => { let mut b = buf_it.next().unwrap(); b.clear(); continue; }
                };

                let t = buf_it.next().unwrap();
                t[0] = t[1]; t[1] = t[2]; t[2] = *c;
                //println!("{}", ascii::raw_to_str(&t).unwrap());
            }
        }

        if all_ciphers_done {
            break;
        }

        println!("-");
        result_i.push(3);
        result_c.push(vec![]);

        if idx >= 2 {
            let trigrams_left = buf.iter().filter(|t| t.len() != 0).count() > 1;

            if trigrams_left {
                let dups = util::dup::<Vec<u8>>(&buf);

                for dup in dups {
                    if dup.0.len() != 0 {
                        println!("dup: {}, {}", ascii::raw_to_str(&dup.0).unwrap(), dup.1);
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


fn xor_keystream(ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>) -> Vec<String> {
    let mut result = Vec::<String>::new();

    for c in ciphers {
        result.push(c.iter().zip(keystream.iter()).map(|(&c, &k)| chr!(c ^ k)).collect());
    }
    result
}


fn break_column(col: &Vec<u8>, candidates: &Vec<u8>, weights: &Vec<u32>) -> Result<u8, err::Error> {
    ctry!(weights.len() > 0 && candidates.len() != weights.len(), "all candidates must have weight");

    let mut dist = Vec::<f32>::new();
    let mut keys: Vec<u8>;
    let no_weight = weights.len() == 0;
    let mut weights_it = weights.iter();

    if candidates.len() == 0 {
        keys = (0 .. 255).collect();
        keys.push(255);
        return Ok(0);
    } else {
        keys = candidates.iter().cloned().collect();
    }

    for i in keys.iter() {
        let xcol = col.iter().map(|&u| u ^ i).collect();
        let d = try!(charfreq::distance_from_base(rts!(&xcol).as_ref()));
        if no_weight {
            dist.push(d);
        } else {
            dist.push(d / *weights_it.next().unwrap() as f32);
        }
    }

    let k = keys[util::min_index(&dist).unwrap()];

    //let s: String = col.iter().map(|&u| chr!(u ^ k as u8)).collect();          
    //println!("col: {}", s);
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

