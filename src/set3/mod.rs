pub mod paddingoracle;
pub mod ctr;
pub mod breakctr;
pub mod breakctr2;

use common::challenge;


pub static challenges: [&'static challenge::Info; 4] = [
    &paddingoracle::info,
    &ctr::info,
    &breakctr::info,
    &breakctr2::info];
 
