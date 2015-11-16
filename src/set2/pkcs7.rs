
use common::cipher::padding;
use common::{challenge, err, util, ascii};


pub static info: challenge::Info = challenge::Info {
    no:         9,
    title:      "Implement PKCS#7 padding",
    help:       "",
    execute_fn: interactive
};



pub fn interactive() -> err::ExitCode {
    let text = rtry!(util::input("enter text"), exit_err!());

    let bsize = rtry!(util::input("enter block size"), exit_err!());

    let blocksize = rtry!(bsize.trim().parse::<usize>(), exit_err!());

    let ptext = rrts!(&rtry!(padding::pkcs7_pad(&rraw!(text.trim()), blocksize), exit_err!()));

    rtry!(padding::print_pkcs7(&ptext, blocksize), exit_err!());

    exit_ok!()
}
