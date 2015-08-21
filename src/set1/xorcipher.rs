use std::fs::File;
use std::io::prelude::{Read, Write};
use std::collections::HashMap;
use std::char;
use std::f32;
use std::env;
use std::io;
use std::fs;

use common::{ascii, err, hex};


static BASE_FREQ_FILENAME: &'static str = "base_frequency";


pub fn compute_base_frequency(sample_filepath: &str) -> Result<Vec<f32>, err::Error> {
    let mut f = etry!(File::open(sample_filepath), "sample data file cannot be opened");
    let mut text = String::new();

    etry!(f.read_to_string(&mut text), "cannot read sample data file..");
    compute_char_frequency(&text)
}

fn generate_base_frequency_file(sample_filepath: &str) -> Result<(), err::Error> {
    let base_freq = etry!(compute_base_frequency(sample_filepath), "could not generate base frequency file");

    let mut f = etry!(File::create(BASE_FREQ_FILENAME), "cannot open base frequency file for writing");

    for freq in base_freq {
        let out = try!(ascii::str_to_raw(&format!("{}\n", freq)));
        f.write(&out);
    }
    println!("base frequency file generated..");
    Ok(())
}


fn compute_char_frequency(text: &str) -> Result<Vec<f32>, err::Error> {
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


fn get_base_freq() -> Result<Vec<f32>, err::Error> {
    match fs::metadata(&BASE_FREQ_FILENAME) {
        Ok(_)   => {}, 
        Err(e)  => return mkerr!("base frequency file not found, please generate it using option: \"genbase <sample text filepath>\"")
    };

    let mut f = etry!(File::open(BASE_FREQ_FILENAME), "could not open base frequency file");
    let mut base_freq = Vec::new();
    
    let mut fstr = String::new();
    etry!(f.read_to_string(&mut fstr), "coul not read base frequency file");

    for val in fstr.split('\n') {
        if val == "" {
            continue;
        }
        let fval = etry!(val.parse::<f32>(),  "error in reading base frequency file: string to float conversion error");
        base_freq.push(fval);
    }
    Ok(base_freq)
}


pub fn decipher(cipher: &str) -> Result<String, err::Error> {
    let mut guess_dist = Vec::new();
    let base_freq = try!(get_base_freq());

    for key in 0..255 {
        let guess = try!(decrypt(cipher, key));
        let guess_freq = try!(compute_char_frequency(&guess));
        
        let mut dist_total = 0f32;

        for i in 0..255 as usize {
            let ch = char::from_u32(i as u32).unwrap();
            dist_total += (base_freq[i] - guess_freq[i]).abs();
        }
        guess_dist.push(dist_total);
    }

    let best_key = try!(min_distance_key(&guess_dist));

    Ok(try!(decrypt(cipher, best_key)))
}


fn min_distance_key(guess_distances: &Vec<f32>) -> Result<u8, err::Error> {
    let mut result_key: u8 = 0;
    let mut min_distance = f32::MAX;

    for i in 0 .. (guess_distances.len() - 1) {
        if guess_distances[i] < min_distance {
            min_distance = guess_distances[i];
            result_key = i as u8;
        }
    }
    Ok(result_key)
}


pub fn decrypt(cipher: &str, key: u8) -> Result<String, err::Error> {
    let raw: Vec<u8> = try!(hex::hex_to_raw(cipher));
    let mut result: Vec<u8> = Vec::new();

    for byte in raw {
       result.push(byte ^ key); 
    }

    ascii::raw_to_string(&result)
}


pub fn i_generate_base_frequency_file() -> u32 {
    let sample_filepath = match env::args().nth(3) {
        Some(v) => v,
        None    => { println!("please provide the path to sample data file"); return 1; }
    };

    match generate_base_frequency_file(&sample_filepath) {
        Ok(_)   => 0,
        Err(_)  => 1
    }
}


pub fn i_decipher() -> u32 {
    println!("enter the hex string to be deciphered: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    match decipher(&input.trim()) {
        Ok(v)   => { println!("{}", v); 0 },
        Err(e)  => { println!("{}", e); 1 }
    }
}


pub fn interactive() -> u32 {
    match env::args().nth(2) {
        Some(v) => match v.as_ref() {
                        "genbase"   => i_generate_base_frequency_file(),
                        _           => i_decipher()
                   },
        None    => i_decipher()
    }
}

