
use common::cipher::padding;
use common::{challenge, err, ascii, util};


pub static info: challenge::Info = challenge::Info {
    no:         15,
    title:      "PKCS#7 padding validation",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> err::ExitCode {
    let ptext = rtry!(util::input("enter padded text (\\x?? for hex value of char)"), exit_err!());

   let blocksize_str = rtry!(util::input_d("enter block size", "16"), exit_err!());

    let blocksize = rtry!(blocksize_str.trim().parse::<usize>(), exit_err!());

    let phtext = rtry!(ascii::scan_hex(&ptext), exit_err!());

    let raw = rtry!(ascii::str_to_raw(&phtext.trim()), exit_err!());

    match padding::pkcs7_detect(&raw, blocksize) {
        Ok(v)  => println!("padding length = {}", v),
        Err(e) => println!("{}", e)
    }
    exit_ok!()
}

