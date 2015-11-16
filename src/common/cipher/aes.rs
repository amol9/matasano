use std::cell::RefCell;

extern crate crypto;
use self::crypto::{buffer, aes, blockmodes};
use self::crypto::symmetriccipher::Decryptor;
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

extern crate rand;
use self::rand::Rng;

use common::{err, xor, end};
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


const Ctr: Cipher = Cipher {
    encrypt_fn: edcrypt_ctr,
    decrypt_fn: edcrypt_ctr
};


#[derive(PartialEq)]
pub enum BlockMode {
    Ecb,
    Cbc
}


pub struct Mode {
    pub keysize:        Option<aes::KeySize>,
    pub blocksize:      Option<usize>,
    pub padding:        Option<padding::Mode>,
    pub cipher:         Cipher,
    pub blockmode:      Option<BlockMode>,
}


impl Mode {
    fn pad(&self, input: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        match self.padding {
            Some(ref p) => (p.pad_fn)(&input, self.blocksize.unwrap()),
            None        => mkerr!("no padding is defined for this mode")
        }
    }


    fn unpad(&self, input: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        match self.padding {
            Some(ref p) => (p.unpad_fn)(&input, self.blocksize.unwrap()),
            None        => mkerr!("no padding is defined for this mode")
        }
    }


    fn encrypt(&self, input: &Vec<u8>, key: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        (self.cipher.encrypt_fn)(&input, &key, &self)
    }


    fn decrypt(&self, input: &Vec<u8>, key: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        (self.cipher.decrypt_fn)(&input, &key, &self)
    }
}


pub const ecb_128_pkcs7: Mode = Mode {
    keysize:    Some(aes::KeySize::KeySize128),
    blocksize:  Some(16),
    padding:    Some(padding::Pkcs7),
    cipher:     Ecb,
    blockmode:  Some(BlockMode::Ecb)
};


pub const cbc_128_pkcs7: Mode = Mode {
    keysize:    Some(aes::KeySize::KeySize128),
    blocksize:  Some(16),
    padding:    Some(padding::Pkcs7),
    cipher:     Cbc,
    blockmode:  Some(BlockMode::Cbc)
};


pub const ctr_128: Mode = Mode {
    keysize:    None,
    blocksize:  None,
    padding:    None,
    cipher:     Ctr,
    blockmode:  None
};


pub fn encrypt(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let result: Vec<u8>;
    if mode.blockmode == Some(BlockMode::Ecb) {
        let padded = try!(mode.pad(&input));
        result = try!(mode.encrypt(&padded, &key));
    } else {
        result = try!(mode.encrypt(&input, &key));
    }
    Ok(result)

}


pub fn decrypt(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let result = try!(mode.decrypt(input, key));
    if mode.blockmode == Some(BlockMode::Ecb) {
        return mode.unpad(&result);
    }
    Ok(result)
}


pub fn encrypt_ecb(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
     let mut encryptor = aes::ecb_encryptor(mode.keysize.unwrap(), key, blockmodes::NoPadding);

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
    let mut decryptor = aes::ecb_decryptor(mode.keysize.unwrap(), key, blockmodes::NoPadding);

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
        Ok(output)
}


pub fn encrypt_cbc(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut block_cipher = try!(iv(&mode));
    let input_iter = input.chunks(mode.blocksize.unwrap());
    let last_block = input.chunks(mode.blocksize.unwrap()).next_back().unwrap();

    let mut output = Vec::<u8>::new();
    output.extend(&block_cipher);

    for block in input_iter {
        let block_v = block.to_vec();

        if block_v == last_block {
           let last = try!(mode.pad(&block_v));
           block_cipher = try!(encrypt_ecb(&try!(xor::xor(&last, &block_cipher)), &key, &mode));
        } else {
           block_cipher = try!(encrypt_ecb(&try!(xor::xor(&block_v, &block_cipher)), &key, &mode));
        }

        output.extend(&block_cipher);
    }
    Ok(output)
}


pub fn iv(mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut rng = rand::thread_rng();
    let iv: Vec<u8> = (0 .. mode.blocksize.unwrap()).map(|_| rng.gen::<u8>()).collect();
    Ok(iv)
}


pub fn decrypt_cbc(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let mut output = Vec::<u8>::new();
    let last_block = input.chunks(mode.blocksize.unwrap()).next_back().unwrap();

    let mut block_it = input.chunks(mode.blocksize.unwrap());
    let mut block_cipher = block_it.next().unwrap().to_vec();

    for block in block_it {
        let block_v = block.to_vec();

        let mut block_plain = try!(xor::xor(&block_cipher, &try!(decrypt_ecb(&block_v, &key, &mode))));
        if block == last_block {
            block_plain = try!(mode.unpad(&block_plain));
        }
        output.extend(&block_plain);
        block_cipher = block_v;
    }
    Ok(output)
}


struct CTRState {
    counter:    u32,
    keystream:  Vec<u8>,
    key_idx:    usize
}


impl CTRState {
    fn new(counter: u32) -> Self {
        CTRState {
            counter:    counter,
            keystream:  Vec::<u8>::new(),
            key_idx:    0
        }
    }
}


pub struct CTR {
    nonce:      u32,
    mode:       Mode,
    key:        Vec<u8>,
    state:      RefCell<CTRState>
}


impl CTR {
    pub fn new(key: &Vec<u8>, nonce: u32) -> Self {
        CTR {
            nonce:      nonce,
            mode:       ecb_128_pkcs7,
            key:        key.clone(),
            state:      RefCell::new(CTRState::new(0u32))
        }
    }


    pub fn gen(&self, input: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        let mut output = Vec::<u8>::new();
        for i in input {
            output.push(try!(self.next_ks_byte()) ^ i);
        }
        Ok(output)
    }


    fn next_ks_byte(&self) -> Result<u8, err::Error> {
        {
            let mut state = self.state.borrow_mut();

            if state.keystream.len() > 0 && state.key_idx < self.mode.blocksize.unwrap() {
                let k = state.keystream[state.key_idx];
                state.key_idx += 1;
                return Ok(k);
            }

            state.keystream.clear();
            state.keystream = try!(encrypt(&rawjoin!(&end::little(self.nonce), &end::little(state.counter)), &self.key, &self.mode));
            state.counter += 1;
            state.key_idx = 0;
        }
       
        self.next_ks_byte()
    }
}


#[allow(unused_variables)]
pub fn edcrypt_ctr(input: &Vec<u8>, key: &Vec<u8>, mode: &Mode) -> Result<Vec<u8>, err::Error> {
    let ctr = CTR::new(&key, 0);
    ctr.gen(&input)
}

