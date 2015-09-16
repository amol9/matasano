
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
        Ok(try!(self.cbox.encrypt(&raw!(self.strings[0]))))
    }


    fn decrypt(&self, cipher: &Vec<u8>) -> Result<bool, err::Error> {
        match self.cbox.decrypt(&cipher) {
            Ok(v)  => Ok(v.len() != cipher.len()),
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

    let mut cipher_block_itr = cipher.chunks(blocksize).rev();
    let mut b2 = cipher_block_itr.next();
    let mut b1 = cipher_block_itr.next();

    let mut last = false;
    while ! last {
        if b1 == None {
            b1 = b2;
            last = true;
        }

        let b12 = rawjoin!(b1.unwrap().into_iter(), b2.unwrap().into_iter());
        println!("b12 len: {}", b12.len());

        let plain_block = try!(break_last_block(&obox, &b12, blocksize));
        println!("plain block: {}", rts!(&plain_block));
        plain.extend(&plain_block);

        b2 = b1;
        b1 = cipher_block_itr.next();
    }
    Ok(rts!(&try!(padding::pkcs7_unpad(&plain, blocksize))))
}


fn break_last_block(obox: &OBox, cipher: &Vec<u8>, blocksize: usize) -> Result<Vec<u8>, err::Error> {
    ctry!(cipher.len() < blocksize * 2, "need at least two blocks of cipher (real or made up) to break the last block");

    let mut byte_index = blocksize - 1;
    let mut padsize = blocksize - byte_index;
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

            match try!(obox.decrypt(&rawjoin!(b1.into_iter(), last_block.clone().into_iter()))) {
                true  => { 
                    plain_rev.push(guess);
                    //print!("guessed right {} ", chr!(guess));
                    if byte_index > 0 {
                        byte_index -= 1;
                        padsize += 1;
                    }
                    break
                },
                false => {}
            };
            if guess == 127 {
                println!("guess failed for byte: {}", blocksize - 1 - i);
            }
        }
        //println!("byte: {}", blocksize - 1 - i);
    }
    Ok(plain_rev.iter().rev().cloned().collect())
}


pub fn interactive() -> err::ExitCode {
    let obox = rtry!(OBox::new(), exit_err!());

    match break_cbc(&obox) {
        Ok(v)  => { match obox.string_valid(&v) {
            true  => println!("\nsuccess !!"),
            false => println!("\nfailed :(")
        };
        exit_ok!() },

        Err(e) => { println!("{}", e);
        exit_err!() }
    }
}

