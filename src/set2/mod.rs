pub mod pkcs7;
pub mod aescbc;
pub mod aesoracle;
pub mod breakaesecb;

use common::challenge;


pub static challenges: [&'static challenge::Info; 4] = [ &pkcs7::info, &aescbc::info, &aesoracle::info, &breakaesecb::info];

