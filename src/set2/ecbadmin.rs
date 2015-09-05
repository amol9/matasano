use std::io;
use std::io::Write;

use common::{err, ascii, base64, util, challenge};
use common::cipher::{aes, oracle, key};


pub static info: challenge::Info = challenge::Info {
    no:         13,
    title:      "ECB cut-and-paste",
    help:       "",
    execute_fn: interactive
};


enum Role {
    user,
    admin
}


struct User {
    email:  String,
    uid:    u32,
    role:   Role
}


impl User {
    fn encode(&self) -> Result<String, err::Error> {
        
    }

    fn decode(&self) -> Result<User, err::Error> {

    }
}


struct AuthBox {
    key:    Vec<u8>,
    mode:   aes::Mode
}

impl AuthBox {
    fn new() -> Result<Self, err::Error> {
        let mode = aes::ecb_128_pkcs7;
        Ok(CipherBox {
            key:    try!(key::random(mode.blocksize)),
            mode:   mode
        })
    }

    fn authenticate(&self, cipher: &Vec<u8>) -> Result<Role, err::Error> {
        let plain = try!(aes::decrypt(&cipher, &self.mode));
        let user = urldecode...
        Ok(user.role)
    }
}

pub fn profile_for(email: &str) -> Result<String, err::Error> {

}


pub fn break_role(role: &str) -> Result<(), err::Error> {

}


pub fn interactive() {
    let mut role = String::new();
    input!("enter role to break into: ", &mut role);


}


    
