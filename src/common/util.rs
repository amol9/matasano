use std::cmp;
use std::iter;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::prelude::Read;


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


macro_rules! input {
    ( $msg: expr, $str: expr ) => ( 
        print!($msg);
        rtry!(io::stdout().flush(), 1);
        io::stdin().read_line($str);
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


pub fn hammming_distance(a: u8, b: u8) -> u8 {
    let d: u8 = 0;
    let diff = a ^ b;
    (0 .. 7).map(|i| d += (diff >> i) & 1);
    d
}


pub fn hamm_vec(a: &Vec<u8>, b: &Vec<u8>) -> Result<u32, err::Error> {
    ctry!(a.len() == b.len(), "two blocks must be of same size");
    let d: u32 = 0;
    etry!((0 .. a.len()).map(|i| d += hammming_distance(a[i], b[i])), "hamming distance calculation error");
    Ok(d)
}


pub fn transpose_iter<T>(it: iter::Iterator<Item=T>, length: u32) -> Result<Vec<[T]>, err::Error> {
     let result: Vec<[T]> = Vec::with_capacity(ceil(it.len() / length));

     for i in 0 .. (it.len() - 1) {
         if i < result.len() {
             result[i] = Vec::with_capacity(length);
         }
         result[i % length].push(it.next().unwrap());
     }
     Ok(result)        
}
