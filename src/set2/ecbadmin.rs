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


struct User {
    email:  String,
    uid:    u32,
    role:   String
}


impl User {
    fn new(email: &str, uid: u32, role: &Role) -> User {
        User {
            email:  email.clone(),
            uid:    uid,
            role:   role
        }
    }


    fn encode(&self) -> Result<String, err::Error> {
        Ok(try!(url::encode(&vec![
            ("email", &self.email),
            ("uid", self.uid),
            ("role", &format!("{:?}", self)])))
    }


    fn decode(param_string: &str) -> Result<User, err::Error> {
        let params = try!(url::decode(&param_string));
        Ok(User {
            email:  params[0][1],
            uid:    params[1][1].parse::<u32>(),
            role:   
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


    fn authenticate(&self, cipher: &Vec<u8>) -> Result<String, err::Error> {
        let plain = try!(aes::decrypt(&cipher, &self.mode));
        let user = User::decode(&plain);
        Ok(user.role)
    }


    fn profile_for(&self, email: &Vec<u8>) -> Response {
        let email_str = ertry!(ascii::raw_to_str(&email));

        let user = User::new(&email, 10, "user");
        let encoded = ertry!(user.encode());
        let enc_raw = ertry!(ascii::str_to_raw(&encoded));
        ertry!(self.cipher.encrypt(&enc_raw))
    }
}


pub fn auth_as_admin(authbox: &AuthBox) -> Result<(), err::Error> {
    let blocksize = try!(detect_blocksize(&authbox, max_blocksize));

    let mut email_name = String::from("a");
    let email_domain = "b.co";

    let email_padsize = blocksize % ("email=".len() + email_name.len() + 1 + email_domain.len());
    for _ in 0 .. email_padsize {
        email_name.push('a');
    }

    let mut email = String::from(email_name);
    email.push('@');
    email.push_str(&email_domain);

    //let email_raw = try!(ascii::str_to_raw(&email));
    let mut suffix = "admin";
    let suffix_raw = try!(ascii::str_to_raw(&suffix));
    let padded_suffix_raw = try!((padding::Pkcs7.unpad_fn)(&suffix_raw, blocksize));
    let padded_suffix = try!(ascii::raw_to_str(&padded_suffix_raw));

    email.push_str(&padded_suffix);

    let token = try!(authbox.profile_for(&email));
    let admin_block = token.chunks(blocksize).nth(1).unwrap();


    email_name = String::from("a");

    let email_padsize = blocksize % ("email=".len() + email_name.len() + 1 + email_domain.len() + "&uid=10&role=".len());
    for _ in 0 .. email_padsize {
        email_name.push('a');
    }

    email = String::from(email_name);
    email.push('@');
    email.push_str(&email_domain);

    let new_token_prefix = String::from("email=");
    new_token_prefix.push_str(&email);
    new_token_prefix.push_str("&uid=10&role=");

    let mut new_token_raw = try!(ascii::str_to_raw(&new_token_prefix));
    new_token_raw.extend(&admin_block);

    let new_token = try!(ascii::raw_to_str(&new_token_raw));


    let role = try!(authbox.authenticate(&new_token));
    match role {
        "admin" => println!("admin access granted !!"),
        _       => println!("admin access denied :(")
    }
}


fn detect_blocksize(authbox: &AuthBox, max_blocksize: usize) -> Result<usize, err::Error> {
    let mut email_name = String::from("a");
    let email_domain = "b.c";

    let len1 = try!(authbox.profile_for(&String::from(&email_name).push_str(&email_domain))).len();

    for _ in 0 .. max_blocksize {
        email_name.push('a');
        let len2 = try!(authbox.profile_for(&String::from(&email_name).push_str(&email_domain))).len();
        if len2 > len1 {
            return Ok(len2 - len1);
        }
    }
    mkerr!("unable to detect cipher block size") 
}


pub fn init_authbox() -> Result<AuthBox, err::Error> {
    Ok(AuthBox::new())
}


pub fn interactive() {
    //let mut role = String::new();
    //input!("enter role to break into: ", &mut role);


}


    
