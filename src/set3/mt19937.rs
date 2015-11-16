
use common::{err, challenge};

pub static info: challenge::Info = challenge::Info {
    no:         21,
    title:      "Implement the MT19937 Mersenne Twister RNG",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    exit_ok!()
}

