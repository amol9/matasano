
use common::{err, ascii};


pub fn pkcs7(input: &mut String, blocksize: usize) -> Result<(), err::Error> {
    ctry!(blocksize == 0, "invalid block size: 0");

    let pad: usize = (blocksize - (input.len() % blocksize)) % blocksize;

    if pad == 0 {
        return Ok(());
    }

    for _ in 0 .. pad {
        input.push(pad as u8 as char);
    }
    Ok(())
}

pub trait Pad {
    fn pad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error>;
    fn unpad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error>;
}

pub struct Mode {
    pub pad_fn:     fn (&Vec<u8>, usize) -> Result<Vec<u8>, err::Error>,
    pub unpad_fn:   fn (&Vec<u8>, usize) -> Result<Vec<u8>, err::Error>
}

pub const Pkcs7: Mode = Mode {
    pad_fn:     pkcs7_pad,
    unpad_fn:   pkcs7_unpad
};

pub const NoPadding: Mode = Mode {
    pad_fn:     no_pad,
    unpad_fn:   no_unpad
};

//takes in raw and returns padded copy
pub fn pkcs7_pad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    ctry!(blocksize == 0, "invalid block size");

    let pad: usize = (blocksize - (block.len() % blocksize)) % blocksize;
    
    if pad == 0 {
        return Ok(block.clone());
    }

    let mut result = block.clone();
    for _ in 0 .. pad {
        result.push(pad as u8);
    }
    Ok(result)
}

pub fn pkcs7_unpad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    let padsize = try!(pkcs7_detect(&block, blocksize));
    //println!("padsize: {}", padsize);

    let mut result = block.clone();
    result.truncate(block.len() - padsize);
    Ok(result)
}

//no padding, just returns the same block
#[allow(unused_variables)]
pub fn no_pad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    Ok(block.clone())
}

#[allow(unused_variables)]
pub fn no_unpad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    Ok(block.clone())
}

pub fn pkcs7_detect(block: &Vec<u8>, blocksize: usize) -> Result<usize, err::Error> {
    ctry!(block.len() == 0, "pkcs7 error: empty block");
    ctry!(block.len() % blocksize != 0, "pkcs7 error: padded data not a multiple of block size");

    let mut i = block.iter().rev();
    let padsize: usize = *i.next().unwrap() as usize;


    if padsize <= blocksize && padsize != 0 {
        for _ in 0 .. (padsize - 1) {
            let c = *i.next().unwrap();
            if c as usize != padsize {
                let valid_chars = ascii::valid_chars();
                if valid_chars.iter().find(|&c| *c == padsize as u8) == None {
                    return mkerr!("invalid padding", err::Type::Padding);
                }
                return Ok(0);
            }
        }           
    } else {
        return Ok(0);
    }
    Ok(padsize)
}

pub fn print_pkcs7(paddedtext: &str, blocksize: usize) -> Result<(), err::Error> {
    let padsize = try!(pkcs7_detect(&paddedtext.as_bytes().to_vec(), blocksize));

    if padsize > 0 {
        let mut pi = paddedtext.chars();
        for _ in 0 .. paddedtext.len() - padsize {
            print!("{}", pi.next().unwrap());
        }

        for _ in 0 .. padsize {
            print!("\\x{:02}", padsize);
        }
        println!("");
    } else {
        println!("{}", paddedtext);
    }
    Ok(())
}

