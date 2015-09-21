pub mod paddingoracle;
pub mod ctr;
pub mod breakctr;

use common::challenge;


pub static challenges: [&'static challenge::Info; 3] = [
    &paddingoracle::info,
    &ctr::info,
    &breakctr::info];
 
