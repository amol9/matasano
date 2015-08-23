use std::fs::File;
use std::io::prelude::{Read, Write};
use std::f32;
use std::fs;

use common::{err, ascii};


static BASE_FREQ_FILENAME: &'static str = "base_freq";


pub fn compute_base_frequency(sample_filepath: &str) -> Result<Vec<f32>, err::Error> {
    let mut f = etry!(File::open(sample_filepath), "sample data file cannot be opened");
    let mut text = String::new();

    etry!(f.read_to_string(&mut text), "cannot read sample data file..");
    compute_char_frequency(&text)
}

pub fn generate_base_frequency_file(sample_filepath: &str) -> Result<(), err::Error> {
    let base_freq = etry!(compute_base_frequency(sample_filepath), "could not generate base frequency file");

    let mut f = etry!(File::create(BASE_FREQ_FILENAME), "cannot open base frequency file for writing");

    for freq in base_freq {
        let out = try!(ascii::str_to_raw(&format!("{}\n", freq)));
        f.write(&out);
    }
    println!("base frequency file generated..");
    Ok(())
}


pub fn compute_char_frequency(text: &str) -> Result<Vec<f32>, err::Error> {
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


pub fn get_base_freq() -> Result<Vec<f32>, err::Error> {
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

