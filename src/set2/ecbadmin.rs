use std::io;
use std::io::Write;

use common::{err, ascii, base64, util, challenge, url};
use common::cipher::{aes, key, padding};


const max_blocksize: usize = 32;


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
    fn new(email: &str, uid: u32, role: &str) -> User {
        User {
            email:  String::from(email),
            uid:    uid,
            role:   String::from(role)
        }
    }


    fn encode(&self) -> Result<String, err::Error> {
        Ok(try!(url::encode(&vec![
            ("email", &self.email),
            ("uid", &format!("{}", self.uid)),
            ("role", &self.role)])))
    }


    fn decode(param_string: &str) -> Result<User, err::Error> {
        let params = try!(url::decode(&param_string));
        Ok(User {
            email:  params[0].1.clone(),
            uid:    etry!(params[1].1.parse::<u32>(), "conversion error"),
            role:   params[2].1.clone() })
    }
}


pub struct AuthBox {
    key:    Vec<u8>,
    mode:   aes::Mode
}


impl AuthBox {
    fn new() -> Result<Self, err::Error> {
        let mode = aes::ecb_128_pkcs7;
        Ok(AuthBox {
            key:    try!(key::random(mode.blocksize)),
            mode:   mode
        })
    }


    fn authenticate(&self, cipher: &Vec<u8>) -> Result<String, err::Error> {
        let plain_raw = try!(aes::decrypt(&cipher, &self.key, &self.mode));
        let plain_str = try!(ascii::raw_to_str(&plain_raw));
        let user = try!(User::decode(&plain_str));
        Ok(user.role)
    }


    fn profile_for(&self, email: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        let email_str = try!(ascii::raw_to_str(&email));

        let user = User::new(&email_str, 10, "user");
        let encoded = try!(user.encode());
        let enc_raw = try!(ascii::str_to_raw(&encoded));
        Ok(try!(aes::encrypt(&enc_raw, &self.key, &self.mode)))
    }
}


pub fn auth_as_admin(authbox: &AuthBox) -> Result<bool, err::Error> {
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

    let token = try!(authbox.profile_for(&raw!(&email)));
    let admin_block = token.chunks(blocksize).nth(1).unwrap();


    email_name = String::from("a");

    let email_padsize = blocksize % ("email=".len() + email_name.len() + 1 + email_domain.len() + "&uid=10&role=".len());
    for _ in 0 .. email_padsize {
        email_name.push('a');
    }

    email = String::from(email_name);
    email.push('@');
    email.push_str(&email_domain);

    let mut new_token_prefix = String::from("email=");
    new_token_prefix.push_str(&email);
    new_token_prefix.push_str("&uid=10&role=");

    let mut new_token_raw = try!(ascii::str_to_raw(&new_token_prefix));
    new_token_raw.extend(admin_block);

    //let new_token = try!(ascii::raw_to_str(&new_token_raw));


    let role = try!(authbox.authenticate(&new_token_raw));
    match role.as_ref() {
        "admin" => { println!("admin access granted !!"); Ok(true) },
        _       => { println!("admin access denied :("); Ok(false) }
    }
}


fn detect_blocksize(authbox: &AuthBox, max: usize) -> Result<usize, err::Error> {
    let mut email_name = String::from("a");
    let email_domain = "b.c";

    let len1 = try!(authbox.profile_for(&raw!(strjoin!(&email_name, &email_domain).as_ref()))).len();

    for _ in 0 .. max {
        email_name.push('a');
        let len2 = try!(authbox.profile_for(&raw!(strjoin!(&email_name, &email_domain).as_ref()))).len();
        if len2 > len1 {
            return Ok(len2 - len1);
        }
    }
    mkerr!("unable to detect cipher block size") 
}


pub fn init_authbox() -> Result<AuthBox, err::Error> {
    Ok(try!(AuthBox::new()))
}


pub fn interactive() -> err::ExitCode {
    //let mut role = String::new();
    //input!("enter role to break into: ", &mut role);

    let authbox = rtry!(init_authbox(), exit_err!());
    rtry!(auth_as_admin(&authbox), exit_err!());
    exit_ok!()
}
    
