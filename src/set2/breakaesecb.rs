
use common::{err, ascii, base64};
use common::cipher::{aes, oracle};


struct CipherBox {
    key:    Vec<u8>,
    data:   Vec<u8>,
    mode:   aes::Mode
}


impl CipherBox {
    fn new(&self, data: &str, mode: &aes::Mode) -> Result<Self, err::Error> {
        Ok(CipherBox {
            key:    try!(key::random(mode.blocksize)),
            data:   try!(ascii::str_to_raw(&data)),
            mode:   mode
        })
    }

    fn gen(&self, prefix: &str) -> Result<Vec<u8>, err::Error> {
        let mut final_input = try!(ascii::str_to_raw(&prefix));
        final_input.extend(&self.data);

        aes::encrypt(&final_input, &self.key, &self.mode)
    }
}


pub fn break_aes_ecb(cipherbox: &CipherBox) -> Result<String, err::Error> {
    let blocksize = try!(detect_block_size(&cipherbox, max_blocksize));
    ctry!(!try!(oracle::detect_aes_ecb(&try!(cipherbox.gen(&String::from(&[65, 2*blocksize]))), blocksize)),
        "cipher is not aes ecb, can't break with this module");

    let range: Range<u8> = Range{ start: 1, end: 127 };

    let mut plaintext = String::new();
    let mut block_no: usize = 0;

    for i in ... {
        let prefix = String::from(&[65, 15]);

        let cipher = try!(cipherbox.gen(&prefix));

        let dict = try!(make_dict(&prefix, &cipherbox, &range));

        let cipher_block = cipher.chunks().nth(block_no);

        let m = dict.chunks().filter(|&ch| ch == cipher_block).collect();
        ctry!(m.len() == 0 || m.len() > 1, format!("no match or multiple matches for character at pos: {}", i));

        let dec_char = m
        
        plaintext.push(dec_char);

    }
    Ok(plaintext)
}


pub fn make_dict(prefix: &str, cipherbox: &CipherBox, range: &Range<u8>) -> Result<Vec<Vec<u8>>, err::Error> {
    let mut dict = Vec::<Vec<u8>>::new();
    let mut plain = String::from(&prefix);

    for i in range {
        plain.push(i as char);
        let cipher = try!(cipherbox.gen(&plain));
        dict.push(cipher);
        plain.pop();
    }
    Ok(dict)
}


pub fn detect_block_size(cipherbox: &CipherBox, max: usize) -> Result<usize, err::Error> {
    let len1 = try!(cipherbox.gen("")).len();

    let mut prefix = String::from("A");
    for i in 0 .. max {
        let len2 = try!(cipherbox.gen(&prefix)).len();
        if len2 > len1 {
            return Ok(len2 - len1);
        }
        prefix.push('A');
    }
    mkerr!("failed to detect cipher block size")
}


pub fn init_cipherbox(filepath: &str) -> Result<CipherBox, err::Error> {
    let plainbase64 = try!(util::read_file_to_str(&filepath));
    let plainraw = try!(base64::base64_to_raw(&try!(ascii::filter_whitespace(&plainbase64))));
    let plaintext = try!(ascii::raw_to_str(&plainraw));

    CipherBox::new(&plaintext, &aes::ecb_128_pkcs7)
}


pub fn interactive() -> u32 {
    let input_filepath = match env::args().nth(2) {
        Some(v) => v,
        None    => { println!("please specify input data (base64 encoded) filepath"); return 1; }
    };

    let cipherbox = rtry!(init_cipherbox(&input_filepath), 1);
    let plaintext = rtry!(break_aes_ecb(&cipherbox), 1);

    println!("{}", plaintext);
    0
}
