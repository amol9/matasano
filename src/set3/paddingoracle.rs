
extern crate rand;
use self::rand::Rng;

use common::{err, challenge, hex, ascii};
use common::cipher::cipherbox as cb;
use common::cipher::{aes, key, padding};


pub static info: challenge::Info = challenge::Info {
    no:         17,
    title:      "The CBC padding oracle",
    help:       "",
    execute_fn: interactive
};


struct OBox {
    cbox:       CipherBox,
    strings:    Vec<String>
}


impl OBox {
    fn new() -> OBox {
        OBox {
            cbox:       cb::CipherBox::new(vec![], aes::cbc_128_pkcs7),
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
        }
    }


    fn get_cipher(&self) -> Result<Vec<u8>, err::Error> {
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen::<usize>() % 10;
        Ok(try!(self.cbox.encrypt(self.strings[rand_idx])))
    }


    fn decrypt(&self, cipher: &Vec<u8>) -> Result<bool, err::Error> {
        Ok(try!(self.cbox.decrypt(&cipher)))
    }


    fn valid_string(&self, input: &str) -> bool {
        self.strings.find(|s| s == input) != None
    }
}


pub fn break_cbc(obox: &OBox) -> Result<String, err::Error> {
    let cipher = try!(obox.get_cipher());
    let mut plain = Vec::<u8>::new();

    let cipher_block_itr = cipher.chunks(blocksize).rev();
    let mut b2 = cipher_block_itr.next();
    let mut b1 = cipher_block_itr.next();

    while b2 != None {
        if b1 == None {
            b1 = vec!['A', blocksize];
        }

        let b12 = rawjoin!(&b1, &b2);
        let plain_block = try!(break_last_block(&obox, &b12, blocksize));
        plain.extend(&plain_block);

        b1 = b2;
        b2 = cipher_block_itr.next();
    }
    Ok(rts!(try!(padding::pkcs7_unpad(&plain, blocksize))))
}


fn break_last_block(obox: &OBox, cipher: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    ctry!(cipher.len() >= blocksize * 2, "need at least two blocks of cipher (real or made up) to break the last block");

    byte_index = blocksize - 1;
    padsize = blocksize - byte_index;
    let mut plain_rev = Vec::<u8>::new();

    let block_iter = cipher.chunks(blocksize).rev();
    let last_block = block_iter().next().unwrap();
    let sec_last_block = block_iter().next().unwrap();

    for i in 0 .. blocksize {
        for guess in 0 .. 255 {
            let mut b1 = sec_last_block.clone();
            plain_rev.iter().zip(b1.iter().rev()).map(|p, c| c ^= p ^ padsize);

            b1[byte_index] ^= guess ^ padsize;

            b1.extend(last_block);
            match try!(obox.decrypt(&b1)) {
                true  => { 
                    plain.push(guess);
                    byte_index += 1;
                    padsize = blocksize - byte_index;
                    break
                },
                false => {}
            };
        }
    }
    Ok(plain_rev.iter().rev().collect())
}


pub fn interactive() -> err::ExitCode {
    let obox = OBox::new();

    match break_cbc(&obox) {
        Ok(v)  => { match obox.valid_string(&v) {
            true  => println!("\nsuccess !!"),
            false => println!("\nfailed :(")
        };
        exit_ok!() },

        Err(e) => { println!("{}", e);
        exit_err!() }
    }
}

