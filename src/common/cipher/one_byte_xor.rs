use std::char;
use std::f32;

use common::{err, ascii, hex, charfreq, util};


pub struct Guess {
    pub plain:      String,
    pub key:        u8,
    pub distance:   f32
}


pub fn try_decipher(cipher: &str) -> Result<Guess, err::Error> {
    let mut guess_dist = Vec::new();
    let base_freq = try!(charfreq::get_base_freq());

    for key in 0..255 {
        let guess = try!(decrypt(cipher, key));
        let guess_freq = try!(charfreq::compute_char_frequency(&guess));
        
        let mut dist_total = 0f32;

        for i in 0..255 as usize {
            let ch = char::from_u32(i as u32).unwrap();
            dist_total += (base_freq[i] - guess_freq[i]).abs();
        }
        guess_dist.push(dist_total);
    }

    let best_key = util::min_index(&guess_dist).unwrap() as u8;

    Ok(Guess {
        plain:      try!(decrypt(cipher, best_key)),
        key:        best_key,
        distance:   guess_dist[best_key as usize]
    })
}


pub fn decrypt(cipher: &str, key: u8) -> Result<String, err::Error> {
    let raw: Vec<u8> = cipher.chars().map(|c| c as u8).collect();                     //try!(hex::hex_to_raw(cipher));
    let mut result: Vec<u8> = Vec::new();

    for byte in raw {
       result.push(byte ^ key); 
    }

    ascii::raw_to_str(&result)
}

