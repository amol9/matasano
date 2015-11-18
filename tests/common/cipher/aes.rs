
use matasano::common::cipher::aes;
use matasano::common::ascii;


enum Op {
    Encrypt,
    Decrypt
}

enum Mode {
    Ecb,
    Cbc,
    Ctr
}


struct Test {
    mode:       Mode,
    blocksize:  usize
}


impl Test {
    fn new(mode: Mode) -> Self {
        Test { 
            mode:       mode,
            blocksize:  16
        }
    }


    fn encdec(&self, input: &str, key: &str, op: Op) -> Option<String> {
        let ir = rn!(ascii::str_to_raw(&input));
        let kr = rn!(ascii::str_to_raw(&key));
        let or: Vec<u8>;
        let olen: usize;

        match self.mode {
            Mode::Ecb | Mode::Cbc => {
                let mode = match self.mode {
                    Mode::Ecb => {
                        olen = (input.len() as f32 / self.blocksize as f32).ceil() as usize * self.blocksize;
                        aes::ecb_128_pkcs7
                    },
                    Mode::Cbc => {                                                                          // + iv length           
                        olen = (input.len() as f32 / self.blocksize as f32).ceil() as usize * self.blocksize + self.blocksize; 
                        aes::cbc_128_pkcs7
                    },
                    _         => unreachable!()
                };

                match op {
                    Op::Encrypt => or = rn!(aes::encrypt(&ir, &kr, &mode)),
                    Op::Decrypt => or = rn!(aes::decrypt(&ir, &kr, &mode)),
                };
            },

            Mode::Ctr => {
                let ctr = aes::CTR::new(&kr, 0);
                or = rn!(ctr.gen(&ir));
                olen = input.len();
            }
        };

        assert!((ir.len() == 0 && or.len() == 0) || or != ir);
        match op {
            Op::Encrypt => assert_eq!(or.len(), olen),
            Op::Decrypt => assert!(or.len() <= input.len())
        }

        println!("{}", rn!(ascii::raw_to_str(&or)));
        Some(rn!(ascii::raw_to_str(&or)))
    }


    fn enc(&self, input: &str, key: &str) -> String {
        self.encdec(&input, &key, Op::Encrypt).unwrap()
    }


    fn dec(&self, input: &str, key: &str) -> String {
        self.encdec(&input, &key, Op::Decrypt).unwrap()
    }


    fn enc_dec(&self, plain: &str, key: &str) {
        let cipher: String;

        cipher = self.enc(&plain, &key);
        assert_eq!(self.dec(&cipher, &key), plain)
    }
}


#[test]
fn test_cbc_128_pkcs7() {
    let test = Test::new(Mode::Cbc);

    test.enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test.enc_dec("hello", "YELLOW SUBMARINE");
    test.enc_dec("a", "YELLOW SUBMARINE");
}


#[test]
fn test_ecb_128_pkcs7() {
    let test = Test::new(Mode::Ecb);

    test.enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test.enc_dec("hello", "YELLOW SUBMARINE");
    test.enc_dec("a", "YELLOW SUBMARINE");
    //test.enc_dec("", "YELLOW SUBMARINE");
}


#[test]
fn test_ctr() {
    let test = Test::new(Mode::Ctr);

    test.enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test.enc_dec("hello", "YELLOW SUBMARINE");
    test.enc_dec("a", "YELLOW SUBMARINE");
    test.enc_dec("", "YELLOW SUBMARINE");
}

