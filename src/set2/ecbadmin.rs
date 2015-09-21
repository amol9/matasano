
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
            ("email",   &self.email),
            ("uid",     &format!("{}", self.uid)),
            ("role",    &self.role)])))
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
            key:    try!(key::random(mode.blocksize.unwrap())),
            mode:   mode
        })
    }


    fn authenticate(&self, cipher: &Vec<u8>) -> Result<String, err::Error> {
        let plain_raw = try!(aes::decrypt(&cipher, &self.key, &self.mode));
        let plain_str = rts!(&plain_raw);
        let user = try!(User::decode(&plain_str));

        Ok(user.role)
    }


    fn profile_for(&self, email: &Vec<u8>) -> Result<Vec<u8>, err::Error> {
        let email_str = rts!(&email);

        let user = User::new(&email_str, 10, "user");
        let encoded = try!(user.encode());
        let enc_raw = raw!(&encoded);

        Ok(try!(aes::encrypt(&enc_raw, &self.key, &self.mode)))
    }
}


pub fn auth_as_admin(authbox: &AuthBox) -> Result<bool, err::Error> {
    let blocksize = try!(detect_blocksize(&authbox, max_blocksize));

    //step1: get cipher block for "admin+padding"

    let mut email_name = String::from("a");
    let email_domain = "b.co";

    let len_email = "email=".len() + email_name.len() + 1 + email_domain.len();
    let email_padsize = (blocksize - (len_email % blocksize)) % blocksize;
    email_name.push_str(strn!('a', email_padsize).as_ref());

    let email = strjoin!(&email_name, "@", &email_domain);                              //email that ends on a block boundary

    let padded_suffix_raw = try!((padding::Pkcs7.pad_fn)(&raw!("admin"), blocksize));   //"admin+padding"
    let mut email_raw = raw!(&email);
    email_raw.extend(&padded_suffix_raw);                                               //email + "admin+padding"

    let token = try!(authbox.profile_for(&email_raw));
    let admin_block = token.chunks(blocksize).nth(1).unwrap().to_vec();                 //extract cipher block for "admin+padding"

    //step2: get cipher block(s) for encoded string upto role=

    email_name = String::from("a");

    let len_upto_role = ("email=".len() + email_name.len() + 1 + email_domain.len() + "&uid=10&role=".len());
    let email_padsize = (blocksize - (len_upto_role % blocksize)) % blocksize;

    email_name.push_str(strn!('a', email_padsize).as_ref());
                                                                                                
    let token2 = try!(authbox.profile_for(&raw!(strjoin!(&email_name, "@", &email_domain).as_ref())));

    let mut new_token = token2.chunks(len_upto_role + email_padsize).next().unwrap().to_vec();  //get token upto role=

    //step3: append the "admin+padding" cipher block from step1 to partial token from step 2

    new_token.extend(&admin_block);

    //step4: get the power !!

    let role = try!(authbox.authenticate(&new_token));

    match role.as_ref() {
        "admin" => { println!("admin access granted !!"); Ok(true) },
        _       => { println!("admin access denied :("); Ok(false) }
    }
}


fn detect_blocksize(authbox: &AuthBox, max: usize) -> Result<usize, err::Error> {
    let mut email_name = String::from("a");
    let email_domain = "b.co";

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
    let authbox = rtry!(init_authbox(), exit_err!());
    rtry!(auth_as_admin(&authbox), exit_err!());
    exit_ok!()
}
    
