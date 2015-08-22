use std::io;

use common::{cipher, err};


pub fn encrypt_str(plain: &str, key: &str) -> Result<String, err::Error> {
    let plain_raw = try!(hex::str_to_raw(plain));

}


pub fn interactive() -> u32 {


}
