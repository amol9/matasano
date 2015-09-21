use std::collections::HashMap;

use common::{err, challenge, hex, ascii};
use common::cipher::cipherbox as cb;
use common::cipher::{aes, key};


pub static info: challenge::Info = challenge::Info {
    no:         16,
    title:      "CBC bitflipping attacks",
    help:       "",
    execute_fn: interactive
};


const escape_chars: [char; 3] = [';', '=', ' '];


pub struct AuthBox {
    key:            Vec<u8>,
    mode:           aes::Mode,
    escape_map:     HashMap<String, String>
}


impl AuthBox {
    fn new() -> Result<Self, err::Error> {
        let mode = aes::cbc_128_pkcs7;
        let mut escape_map = HashMap::new();

        for c in escape_chars.iter() {
            let mut chex = String::from("%");
            chex.push(try!(hex::u8_to_hex_char::<hex::upper>(*c as u8 >> 4)));
            chex.push(try!(hex::u8_to_hex_char::<hex::upper>(*c as u8 & 0xF)));
            escape_map.insert(c.to_string(), chex);
        }

        Ok(AuthBox {
            key:        try!(key::random(mode.blocksize.unwrap())),
            mode:       mode,
            escape_map: escape_map
        })
    }


    fn authenticate(&self, cipher: &Vec<u8>) -> Result<&str, err::Error> {
        let plain_raw = try!(aes::decrypt(&cipher, &self.key, &self.mode));
        let plain_str = rts!(&plain_raw);

        println!("decrypted: {}", plain_str);

        let role = match plain_str.find(";admin=true") {
            Some(v) => "admin",
            None    => "other"
        };

        Ok(role)
    }


    fn submit(&self, comment1: &str, userdata: &str, comment2: &str) -> Result<Vec<u8>, err::Error> {
        let raw = raw!(strjoin!("comment1=", &self.escape(&comment1),
                                ";userdata=", &self.escape(&userdata),
                                ";comment2=", &self.escape(&comment2)).as_ref());

        println!("submitted: {}", rts!(&raw));

        Ok(try!(aes::encrypt(&raw, &self.key, &self.mode)))
    }


    fn escape(&self, input: &str) -> String {
        let mut output = String::from(input);
        
        for (c, r) in &self.escape_map {
            output = output.replace(&c, &r);
        }
        output
    }
}


pub fn auth_as_admin(authbox: &AuthBox) -> Result<bool, err::Error> {
    let blocksize = 16;

    let comment1 = "cooking MCs";
    let comment2 = " like a pound of bacon";

    let len1 = strjoin!("comment1=", comment1, ";userdata=").len() + 2;
    let padsize = blocksize - (len1 % blocksize) + blocksize;
    let padstr = strn!('A', padsize);

    let userdata = strjoin!(&padstr, &chr!(';' as u8 ^ 1).to_string(), "admin", &chr!('=' as u8 ^ 1).to_string(), "true");

    let flip_idx1 = len1 + padsize; 
    let flip_idx2 = flip_idx1 + "admin=".len();

    let mut cipher = try!(authbox.submit(&comment1, &userdata, &comment2));
    cipher[flip_idx1] ^= 1;
    cipher[flip_idx2] ^= 1;

    match authbox.authenticate(&cipher) {
        Ok("admin") => { println!("admin access granted !!"); Ok(true) },
        _           => { println!("admin access denied :("); Ok(false) }
    }
}


pub fn init_authbox() -> Result<AuthBox, err::Error> {
    Ok(try!(AuthBox::new()))
}


pub fn interactive() -> err::ExitCode {
    let authbox = rtry!(init_authbox(), exit_err!());
    rtry!(auth_as_admin(&authbox), exit_err!());

    exit_ok!()
}

