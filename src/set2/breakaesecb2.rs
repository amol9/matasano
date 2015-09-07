use std::env;
use std::io;
use std::io::Write;


use common::{err, ascii, base64, util, challenge};
use common::cipher::{aes, oracle, key};


pub static info: challenge::Info = challenge::Info {
    no:         14,
    title:      "Byte-at-a-time ECB decryption (Harder)",
    help:       "param1: path to base 64 encoded plain text file (to be used as target data)",
    execute_fn: interactive
};


const max_blocksize: usize = 32;
const max_random_data_length = 512;




fn interactive() -> err::ExitCode {
    0

}
