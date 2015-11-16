
use common::{err, challenge, ascii, base64, util};
use common::cipher::aes;



pub static info: challenge::Info = challenge::Info {
    no:         18,
    title:      "Implement CTR, the stream cipher mode",
    help:       "",
    execute_fn: interactive
};


pub fn ctr_crypt(input_b64: &str, key: &str) -> Result<String, err::Error> {
    let ctr = aes::CTR::new(&raw!(&key), 0);
    Ok(rts!(&try!(ctr.gen(&b64tr!(&input_b64)))))
}


pub fn interactive() -> err::ExitCode {
    let input_b64 = rtry!(util::input("enter input (base64)"), exit_err!());
    
    let key = rtry!(util::input_d("enter key", "YELLOW SUBMARINE"), exit_err!());

    let output = rtry!(ctr_crypt(&input_b64.trim(), &key.trim()), exit_err!());

    println!("{}", output);
   
    exit_ok!()
}
