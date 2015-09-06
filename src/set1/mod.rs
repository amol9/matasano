pub mod hextobase64;
pub mod fixedxor;
pub mod xorcipher;
pub mod detectxorcipher;
pub mod rptxorcipher;
pub mod breakrptxor;
pub mod aesdecrypt;
pub mod detectaesecb;

use common::challenge;


pub static challenges: [&'static challenge::Info; 8] = [
    &hextobase64::info,
    &fixedxor::info,
    &xorcipher::info,
    &detectxorcipher::info,
    &rptxorcipher::info,
    &breakrptxor::info,
    &aesdecrypt::info,
    &detectaesecb::info];

