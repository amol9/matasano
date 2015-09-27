
use common::{err, challenge, ascii, base64, util, charfreq};
use common::cipher::aes;


pub static info: challenge::Info = challenge::Info {
    no:         20,
    title:      "Break fixed-nonce CTR statistically",
    help:       "param1: path to file containing base64 encoded plain strings",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    exit_ok!()
}

