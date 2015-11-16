pub mod paddingoracle;
pub mod ctr;
pub mod breakctr;
pub mod breakctr2;
pub mod mt19937;

use common::challenge;


pub static challenges: [&'static challenge::Info; 5] = [
    &paddingoracle::info,
    &ctr::info,
    &breakctr::info,
    &breakctr2::info,
    &mt19937::info];
 
