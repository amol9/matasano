pub mod pkcs7;
pub mod aescbc;
pub mod aesoracle;
pub mod breakaesecb;
pub mod ecbadmin;
pub mod breakaesecb2;
pub mod badpadding;
pub mod cbcadmin;

use common::challenge;


pub static challenges: [&'static challenge::Info; 8] = [
    &pkcs7::info,
    &aescbc::info,
    &aesoracle::info,
    &breakaesecb::info,
    &ecbadmin::info,
    &breakaesecb2::info,
    &badpadding::info,
    &cbcadmin::info];

