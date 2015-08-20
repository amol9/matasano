use std::fs::File;
use std::io::prelude::Read;
use std::collections::HashMap;
use std::char;
use std::f32;

use common::{ascii, err, hex};


fn compute_base_frequency() -> Result<Vec<f32>, err::Error> {
    let mut f = etry!(File::open("pg1342_ascii.txt"), "sample data file cannot be opened..");
    let mut text = String::new();

    etry!(f.read_to_string(&mut text), "cannot read sample data file..");
    compute_text_frequency(&text)
}


fn compute_text_frequency(text: &str) -> Result<Vec<f32>, err::Error> {
    let mut freq_map: Vec<f32> = vec![0f32; 256];

    let mut char_count: Vec<u32> = vec![0; 256];

    for c in text.chars() {
        char_count[c as usize] += 1;
    }

    for i in 0..255 {
        freq_map[i as usize] = char_count[i as usize] as f32 / text.len() as f32;
    }
    Ok(freq_map)
}


pub fn decipher(cipher: &str) -> Result<String, err::Error> {
    let mut guess_dist = Vec::new();
    let avg_freq = etry!(compute_base_frequency(), "failed to compute base frequency");

    for key in 0..255 {
        let guess = try!(decrypt(cipher, key));
        let guess_freq = try!(compute_text_frequency(&guess));
        
        let mut dist_total = 0f32;

        for i in 0..255 {
            let ch = char::from_u32(i).unwrap();
            dist_total += (avg_freq[i as usize] - guess_freq[i as usize]).abs();
        }
        guess_dist.push(dist_total / guess.len() as f32);
    }

    let best_key = try!(min_distance_key(&guess_dist));

    Ok(try!(decrypt(cipher, best_key)))
}


fn min_distance_key(guess_distances: &Vec<f32>) -> Result<u8, err::Error> {
    let mut result_key: u8 = 0;
    let mut min_distance = f32::MAX;

    for i in 0 ..(guess_distances.len() - 1) {
        if guess_distances[i] < min_distance {
            min_distance = guess_distances[i];
            result_key = i as u8;
        }
    }
    Ok(result_key)
}


fn decrypt(cipher: &str, key: u8) -> Result<String, err::Error> {
    let raw: Vec<u8> = try!(hex::hex_to_raw(cipher));
    let mut result: Vec<u8> = Vec::new();

    for byte in raw {
       result.push(byte ^ key); 
    }

    ascii::raw_to_string(&result)
}

