
use common::err;


pub fn xor(x: &Vec<u8>, y: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
    ctry!(x.len() == y.len(), "xor error: two blocks must be the same size");
    Ok(x.iter().zip(y.iter()).map(|(a, b)| *a ^ *b).collect())
}

