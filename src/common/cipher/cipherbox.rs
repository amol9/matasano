
extern crate rand;
use self::rand::Rng;

use common::{err, ascii, util, base64};
use common::cipher::{key, aes};

pub struct CipherBox {
    key:            Vec<u8>,
    target_data:    Vec<u8>,
    mode:           aes::Mode,
    random_prefix:  bool,
    max_random_len: usize
}


impl CipherBox {
    pub fn new(target_data: &Vec<u8>, mode: aes::Mode) -> Result<Self, err::Error> {
        Ok(CipherBox {
            key:            try!(key::random(mode.blocksize)),
            target_data:    target_data.clone(),
            mode:           mode,
            random_prefix:  false,
            max_random_len: 0
        })
    }


    pub fn enable_random_prefix(&mut self, max_len: usize) {
        self.random_prefix = true;
        self.max_random_len = max_len;
    }


    pub fn encrypt(&self, input: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        let mut data = Vec::<u8>::new();
        if self.random_prefix {
            data.extend(&try!(self.random_data()));
        }

        data.extend(input);
        data.extend(&self.target_data);

        aes::encrypt(&data, &self.key, &self.mode)
    }


    fn random_data(&self) -> Result<Vec<u8>, err::Error> {
        let mut rng = rand::thread_rng();
        Ok((0 .. rng.gen::<usize>() % self.max_random_len).map(|_| rng.gen::<u8>()).collect())
    }
}


pub fn init_from_file(filepath: &str) -> Result<CipherBox, err::Error> {
    let plain_base64 = try!(util::read_file_to_str(&filepath));
    let clean_base64 = try!(ascii::filter_whitespace(&plain_base64));

    init(&clean_base64)
}


pub fn init(plaintext_base64: &str) -> Result<CipherBox, err::Error> {
    let plainraw = try!(base64::base64_to_raw(&plaintext_base64));
    CipherBox::new(&plainraw, aes::ecb_128_pkcs7)
}


pub fn make_dict(prefix: &Vec<u8>, cipherbox: &CipherBox, max_u8: u8) -> Result<Vec<Vec<u8>>, err::Error> {
    let mut dict = Vec::<Vec<u8>>::new();
    let mut block = prefix.clone();

    for i in 1 .. max_u8 + 1 {
        block.push(i as u8);
        let cipher = try!(cipherbox.encrypt(&block));
        let cipher_block0 = cipher.chunks(block.len()).next().unwrap().to_vec();

        dict.push(cipher_block0);
        block.pop();
    }
    Ok(dict)
}


