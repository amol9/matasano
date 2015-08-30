
use common::err;


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


pub trait Padding {
    fn pad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error>;
    fn unpad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error>;
}


struct Pkcs7 {
}

impl Padding for Pkcs7 {

    //takes in raw and returns padded copy
    pub fn pad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
        ctry!(blocksize == 0 || blocksize < block.len(), "invalid block size");

        let pad: usize = (blocksize - (input.len() % blocksize)) % blocksize;
        
        if pad == 0 {
            return Ok(vec!(block));
        }

        let result = vec!(block);
        for _ in 0 .. pad {
            result.push(pad as u8);
        }
        Ok(result)
    }

    pub fn unpad(block: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {

    }

}


struct NoPadding {
}


impl Padding for NoPadding {

    //no padding, just returns the same block
    pub fn pad(block: &Vec<u8>, blocksize: usize) -> Result<&Vec<u8>, err::Error> {
        Ok(block)
    }

    pub fn unpad(block: &Vec<u8>, blocksize: usize) -> Result<&Vec<u8>, err::Error> {
        Ok(block)
    }

}



pub fn print_pkcs7(paddedtext: &str, blocksize: usize) -> Result<(), err::Error> {
    let mut i = paddedtext.chars().rev();
    let padsize: usize = i.next().unwrap() as usize;

    if padsize < blocksize {
        for _ in 0 .. (padsize - 1) {
            let c = i.next().unwrap();
            if c as usize != padsize {
                return mkerr!("bad padding");
            }
        }

        let mut pi = paddedtext.chars();
        for _ in 0 .. paddedtext.len() - padsize {
            print!("{}", pi.next().unwrap());
        }

        for _ in 0 .. padsize {
            print!("\\x{:02}", padsize);
        }
        println!("");
        return Ok(());
    }

    println!("{}", paddedtext);
    Ok(())
}

