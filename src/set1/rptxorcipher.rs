
use common::cipher::rpt_key_xor as rkx;
use common::{util, challenge, err};


pub static info: challenge::Info = challenge::Info {
    no:         5,
    title:      "Implement repeating-key XOR",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    let plain = rtry!(util::input("enter plain text"), exit_err!());

    let key = rtry!(util::input("enter key"), exit_err!());

    let cipher = rtry!(rkx::encrypt_str(&plain, &key), exit_err!());

    println!("cipher: {}", cipher);

    exit_ok!()
}

