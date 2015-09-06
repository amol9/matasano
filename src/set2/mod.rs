pub mod pkcs7;
pub mod aescbc;
pub mod aesoracle;
pub mod breakaesecb;
pub mod ecbadmin;

use common::challenge;


pub static challenges: [&'static challenge::Info; 5] = [
    &pkcs7::info,
    &aescbc::info,
    &aesoracle::info,
    &breakaesecb::info,
    &ecbadmin::info];

