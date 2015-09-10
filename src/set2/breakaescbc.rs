
use common::{err, util, challenge, ascii};
use common::cipher::cipherbox as cb;


pub static info: challenge::Info = challenge::Info {
    no:         16,
    title:      "CBC bitflipping attacks",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    exit_ok!()
}

