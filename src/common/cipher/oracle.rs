
use common::err;

pub fn detect_aes_ecb(cipherraw: &Vec<u8>, blocksize: usize) -> Result<bool, err::Error> {
    ctry!(cipherraw.len() % blocksize != 0, "cipher not a multiple of block size");

    let mut skip: usize = 1;
    for start_block in cipherraw.chunks(blocksize) {
        for block in cipherraw.chunks(blocksize).skip(skip) {
            if start_block == block {
                return Ok(true);
            }
        }
        skip += 1;
    }
    Ok(false)
}

