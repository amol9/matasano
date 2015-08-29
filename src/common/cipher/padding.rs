
use common::err;


pub fn pkcs7(input: &mut String, blocksize: usize) -> Result<(), err::Error> {
    ctry!(blocksize != 0, "invalid block size: 0");

    let pad: usize = (blocksize - (input.len() % blocksize)) % blocksize;

    if pad == 0 {
        return Ok(());
    }

    for _ in 0 .. pad {
        input.push(pad as u8 as char);
    }
    Ok(())
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

