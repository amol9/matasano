
use common::err;

//a black box for encryption
pub struct Cipher {
    key:    Vec<u8>,
    mode:   aes::Mode
}


impl Cipher {
    fn new(data: &Vec<u8>, mode: aes::Mode) -> Result<Self, err::Error> {
        Ok(CipherBox {
            key:    try!(key::random(mode.blocksize)),
            data:   data.clone(),
            mode:   mode
        })
    }


    fn encrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        aes::encrypt(&data, &self.key, &self.mode)
    }


    fn decrypt(&self, data: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        aes::decrypt(&data, &self.key, &self.mode)
    }
}


