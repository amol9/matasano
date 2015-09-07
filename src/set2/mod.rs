pub mod pkcs7;
pub mod aescbc;
pub mod aesoracle;
pub mod breakaesecb;
pub mod ecbadmin;
pub mod breakaesecb2;

use common::challenge;


pub static challenges: [&'static challenge::Info; 6] = [
    &pkcs7::info,
    &aescbc::info,
    &aesoracle::info,
    &breakaesecb::info,
    &ecbadmin::info,
    &breakaesecb2::info];

