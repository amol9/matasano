use std::env;

extern crate rand;
use self::rand::Rng;

use common::{err, challenge, hex, ascii};
use common::cipher::cipherbox as cb;
use common::cipher::{aes, key, padding};


pub static info: challenge::Info = challenge::Info {
    no:         17,
    title:      "The CBC padding oracle",
    help:       "param1 (optional): number of trials (default = 1)",
    execute_fn: interactive
};


pub struct OBox {
    cbox:       cb::CipherBox,
    strings:    Vec<&'static str>
}


impl OBox {
    pub fn new() -> Result<OBox, err::Error> {
        Ok(OBox {
            cbox:       try!(cb::CipherBox::new(&vec![], aes::cbc_128_pkcs7)),
            strings:    vec![
                "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
                "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
                "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
                "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
                "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
                "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
                "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
                "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
                "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
                "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93" ]
        })
    }


    fn get_cipher(&self) -> Result<Vec<u8>, err::Error> {
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen::<usize>() % 10;
        self.cbox.encrypt(&raw!(self.strings[rand_idx]))
    }


    fn dec_oracle(&self, cipher: &Vec<u8>) -> Result<bool, err::Error> {
        match self.cbox.decrypt(&cipher) {
            Ok(v)  => Ok((v.len() + self.cbox.blocksize()) != cipher.len()),
            Err(e) => match e.errtype {
                err::Type::Padding  => Ok(false),
                _                   => Err(e)
            }
        }
    }


    pub fn string_valid(&self, input: &str) -> bool {
        self.strings.iter().find(|&s| *s == input) != None
    }
}


pub fn break_cbc(obox: &OBox) -> Result<String, err::Error> {
    let blocksize = 16;

    let cipher = try!(obox.get_cipher());
    let mut plain = Vec::<u8>::new();

    let mut cipher_block_itr = cipher.chunks(blocksize);
    let mut b1 = cipher_block_itr.next();
    let mut b2 = cipher_block_itr.next();

    while b1 != None && b2 != None {
        let b12 = rawjoin!(b1.unwrap().into_iter(), b2.unwrap().into_iter());

        let plain_block = try!(break_last_block(&obox, &b12, blocksize));       // break one block at a time
        plain.extend(&plain_block);                                             // by sending it with its predecessor

        b1 = b2;
        b2 = cipher_block_itr.next();
    }

    Ok(rts!(&try!(padding::pkcs7_unpad(&plain, blocksize))))
}


// algorithm reference: https://en.wikipedia.org/wiki/Padding_oracle_attack
//
fn break_last_block(obox: &OBox, cipher: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    ctry!(cipher.len() < blocksize * 2, "need at least two blocks of cipher (real or made up) to break the last block");

    let mut byte_index = blocksize - 1;
    let mut padsize = 1;
    let mut plain_rev = Vec::<u8>::new();

    let mut block_iter = cipher.chunks(blocksize).rev();
    let last_block = block_iter.next().unwrap().to_vec();
    let sec_last_block = block_iter.next().unwrap().to_vec();

    for i in 0 .. blocksize {
        for guess in 0 .. 128 {
            let mut b1: Vec<u8> = sec_last_block.clone();

            for i in 0 .. plain_rev.len() {
                b1[blocksize - 1 - i] ^= plain_rev[i] ^ padsize as u8;
            }

            b1[byte_index] ^= guess ^ padsize as u8;

            if padsize == 1 {                   // step needed for actually padded blocks
                for i in 0 .. byte_index {      // e.g. if actual padding is 4 and by some chance, our guess results in last byte = 4,
                    b1[i] ^= 128;               // instead of 1, padding oracle returns true and we guess wrong
                }                               // so, we flip the highest bit of each of the preceding
            }                                   // bytes so that even if we get last byte = 4, most of the
                                                // bytes previous to it will be >127
                                                
            match try!(obox.dec_oracle(&rawjoin!(b1.into_iter(), last_block.clone().into_iter()))) {
                true  => { 
                    plain_rev.push(guess);
                    if byte_index > 0 {
                        byte_index -= 1;
                        padsize += 1;
                    }
                    break
                },
                false => {}
            };
        }
    }
    Ok(plain_rev.iter().rev().cloned().collect())
}


pub fn interactive() -> err::ExitCode {
    let obox = rtry!(OBox::new(), exit_err!());

    let n = match env::args().nth(2) {
        Some(v) => rtry!(v.parse::<usize>(), exit_err!()),
        None    => 1
    };

    for _ in 0 .. n {
        match break_cbc(&obox) {
            Ok(v)  => match obox.string_valid(&v) {
                true  => println!("{} : success!!", v),
                false => println!("{} : failed:(", v)
            },
            Err(e) => println!("{}", e)
        }
    }
    exit_ok!()
}

