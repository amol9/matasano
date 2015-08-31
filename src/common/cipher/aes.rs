extern crate crypto;
use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::symmetriccipher::Decryptor;
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

use common::{err, xor};
use common::cipher::padding;


pub struct Cipher {
    pub encrypt_fn:     fn(&Vec<u8>, &Vec<u8>, &Mode) -> Result<Vec<u8>, err::Error>,
    pub decrypt_fn:     fn(&Vec<u8>, &Vec<u8>, &Mode) -> Result<Vec<u8>, err::Error>
}


const Ecb: Cipher = Cipher {
    encrypt_fn: encrypt_ecb,
    decrypt_fn: decrypt_ecb
};


const Cbc: Cipher = Cipher {
    encrypt_fn: encrypt_cbc,
    decrypt_fn: decrypt_cbc
};

#[derive(PartialEq)]
pub enum BlockMode {
    ecb,
    cbc
}


pub struct Mode {
    pub keysize:        aes::KeySize,
    pub blocksize:      usize,
    pub padding:        padding::Mode,
    pub cipher:         Cipher,
    pub blockmode:      BlockMode,
}


pub const ecb_128_pkcs7: Mode = Mode {
    keysize:    aes::KeySize::KeySize128,
    blocksize:  16,
    padding:    padding::Pkcs7,
    cipher:     Ecb,
    blockmode:  BlockMode::ecb
};


pub const cbc_128_pkcs7: Mode = Mode {
    keysize:    aes::KeySize::KeySize128,
    blocksize:  16,
    padding:    padding::Pkcs7,
    cipher:     Cbc,
    blockmode:  BlockMode::cbc
};


pub fn encrypt(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut result: Vec<u8>;
    if mode.blockmode == BlockMode::ecb {
        let plain = try!((mode.padding.pad_fn)(&input, mode.blocksize));
        result = try!((mode.cipher.encrypt_fn)(&plain, &key, &mode));
    } else {
        result = try!((mode.cipher.encrypt_fn)(&input, &key, &mode));
    }
    Ok(result)

}


pub fn decrypt(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let result = try!((mode.cipher.decrypt_fn)(input, key, mode));
    if mode.blockmode == BlockMode::ecb {
        return (mode.padding.unpad_fn)(&result, mode.blocksize);
    }
    Ok(result)
}


pub fn encrypt_ecb(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
     let mut encryptor = aes::ecb_encryptor(mode.keysize, key, blockmodes::NoPadding);

        let mut output = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(input);
        let mut buffer = [0; 2048];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true) {
                Ok(v)   => v,
                Err(_)  => return mkerr!("encryption error")
            };
            output.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        Ok(output)
}


pub fn decrypt_ecb(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut decryptor = aes::ecb_decryptor(mode.keysize, key, blockmodes::NoPadding);

        let mut output = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(input);
        let mut buffer = [0; 2048];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = match decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
                Ok(v)   => v,
                Err(_)  => return mkerr!("decryption error")
            };
            output.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        //output = try!((mode.padding.unpad_fn)(&output, mode.blocksize));
        Ok(output)
}


pub fn encrypt_cbc(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut block_cipher = try!(iv(&key, &mode));
    let mut input_iter = input.chunks(mode.blocksize);
    let last_block = input.chunks(mode.blocksize).next_back().unwrap();
    let mut output = Vec::<u8>::new();

    for block in input_iter {
        let block_v = block.to_vec();

        if block_v == last_block {
           let last = try!((mode.padding.pad_fn)(&block_v, mode.blocksize));
           block_cipher = try!(encrypt_ecb(&try!(xor::xor(&last, &block_cipher)), &key, &mode));
        } else {
           block_cipher = try!(encrypt_ecb(&try!(xor::xor(&block_v, &block_cipher)), &key, &mode));
        }

        output.extend(&block_cipher);
    }
    Ok(output)
}


pub fn iv(key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let iv: Vec<u8> = vec![0; mode.blocksize];
    encrypt_ecb(&iv, &key, &mode)
}


pub fn decrypt_cbc(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut block_cipher = try!(iv(&key, &mode));
    let mut output = Vec::<u8>::new();
    let last_block = input.chunks(mode.blocksize).next_back().unwrap();

    for block in input.chunks(mode.blocksize) {
        let block_v = block.to_vec();

        let mut block_plain = try!(xor::xor(&block_cipher, &try!(decrypt_ecb(&block_v, &key, &mode))));
        if block == last_block {
            block_plain = try!((mode.padding.unpad_fn)(&block_plain, mode.blocksize));
        }
        output.extend(&block_plain);
        block_cipher = block_v;
    }
    Ok(output)
}

