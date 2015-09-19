use std::cmp;
use std::iter;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;

use common::err;


const F32_ZERO: f32 = 1.0e-40_f32;


pub fn min_index<T: PartialOrd>(list: &Vec<T>) -> Option<usize> {
    let mut min_value = None;
    let mut min_index: usize = 0;

    let mut index: usize = 0;

    for i in list {
        min_value = match min_value {
            Some(v) => if i < v { min_index = index; Some(i) } else { Some(v) },
            None    => Some(i)
          };
        index += 1;
    }
    Some(min_index)
}


pub fn min_indices<T: PartialOrd>(list: &Vec<T>, count: usize) -> Option<Vec<usize>> {
    let mut result = Vec::new();
    //let result_contains = |idx| 

    for c in 0 .. count {
        let mut min_value = None;
        let mut min_index: usize = 0;

        let mut index: usize = 0;

        for i in list {
            if !result.iter().any(|x| *x == index) {
                min_value = match min_value {
                    Some(v) => if i < v { min_index = index; Some(i) } else { Some(v) },
                    None    => Some(i)
                  };
            }
            index += 1;
        }

        result.push(min_index);
    }
    Some(result)
}


macro_rules! input {
    ( $msg: expr, $str: expr ) => ( 
        print!($msg);
        rtry!(io::stdout().flush(), 1);
        io::stdin().read_line($str);
    );
    
    ( $msg: expr, $str: expr, $default: expr ) => ( 
        {
            print!("{} [{}]: ", $msg, $default);
            rtry!(io::stdout().flush(), 1);
            let r = io::stdin().read_line($str);
            match r {
                Ok(n)  => if n == 1 {
                    (*$str).clear();
                    (*$str).push_str($default)
                },
                Err(e) => {}
            };
        }
    );
}


pub fn read_file_to_str(filepath: &str) -> Result<String, err::Error> {
    match fs::metadata(&filepath) {
        Ok(v)   => {},
        Err(e)  => etry!(Err(e), format!("{} not found", filepath)),
    };

    let mut f = etry!(File::open(&filepath), format!("cannot open {}", filepath));
    let mut text = String::new();
    etry!(f.read_to_string(&mut text), format!("cannot read input {}", filepath));

    Ok(text)
}


pub fn hamming_distance(a: u8, b: u8) -> u8 {
    let mut d: u8 = 0;
    let diff = a ^ b;
    for i in (0 .. 8) {
        d += (diff >> i) & 1;
    }
    d
}


pub fn hamm_vec(a: &Vec<u8>, b: &Vec<u8>) -> Result<u32, err::Error> {
    ctry!(a.len() != b.len(), "two blocks must be of same size");
    let mut d: u32 = 0;
    for i in (0 .. a.len()) {
        d += hamming_distance(a[i], b[i]) as u32;
    }
    Ok(d)
}


pub fn transpose_vec<T: Clone>(input: &Vec<T>, length: u32) -> Result<Vec<Vec<T>>, err::Error> {
    let mut result: Vec<Vec<T>> = Vec::new();
    for _ in 0 .. length {
        result.push(Vec::new());
    }

    let mut i: usize = 0;
    for v in input {
        result[(i as u32 % length) as usize].push(v.clone());
        i += 1;
    }
    Ok(result)        
}


pub fn transpose_str(input: &str, length: u32) -> Result<Vec<String>, err::Error> {
    let mut result = Vec::new();
    for _ in 0 .. length {
        result.push(String::new());
    }

    let mut i: usize = 0;
    for c in input.chars() {
        result[(i as u32 % length) as usize].push(c);
        i += 1;
    }
    Ok(result)
}


pub fn shift_left_and_push(input: &Vec<u8>, byte: u8) -> Result<Vec<u8>, err::Error> {
    let mut input_iter = input.iter();
    input_iter.next();
    let mut result: Vec<u8> = input_iter.cloned().collect();
    result.push(byte);
    Ok(result)
}


macro_rules! printc {
    ( $x : expr ) => ( 
        print!("{}", $x );
        io::stdout().flush(); );
}

