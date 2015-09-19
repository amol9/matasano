pub mod paddingoracle;
pub mod ctr;

use common::challenge;


pub static challenges: [&'static challenge::Info; 2] = [
    &paddingoracle::info,
    &ctr::info];
 
