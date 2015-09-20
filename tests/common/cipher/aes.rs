
use matasano::common::cipher::aes;
use matasano::common::ascii;


enum Op {
    encrypt,
    decrypt
}

enum Mode {
    ecb,
    cbc,
    ctr
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
            Mode::ecb | Mode::cbc => {
                let mode = match self.mode {
                    Mode::ecb => {
                        olen = (input.len() as f32 / self.blocksize as f32).ceil() as usize * self.blocksize;
                        aes::ecb_128_pkcs7
                    },
                    Mode::cbc => {                                                                          // + iv length           
                        olen = (input.len() as f32 / self.blocksize as f32).ceil() as usize * self.blocksize + self.blocksize; 
                        aes::cbc_128_pkcs7
                    },
                    _         => unreachable!()
                };

                match op {
                    Op::encrypt => or = rn!(aes::encrypt(&ir, &kr, &mode)),
                    Op::decrypt => or = rn!(aes::decrypt(&ir, &kr, &mode)),
                };
            },

            Mode::ctr => {
                let mut ctr = aes::CTR::new(&kr, 0);
                or = rn!(ctr.gen(&ir));
                olen = input.len();
            }
        };

        assert!((ir.len() == 0 && or.len() == 0) || or != ir);
        match op {
            Op::encrypt => assert_eq!(or.len(), olen),
            Op::decrypt => assert!(or.len() <= input.len())
        }

        println!("{}", rn!(ascii::raw_to_str(&or)));
        Some(rn!(ascii::raw_to_str(&or)))
    }


    fn enc(&self, input: &str, key: &str) -> String {
        self.encdec(&input, &key, Op::encrypt).unwrap()
    }


    fn dec(&self, input: &str, key: &str) -> String {
        self.encdec(&input, &key, Op::decrypt).unwrap()
    }


    fn enc_dec(&self, plain: &str, key: &str) {
        let cipher: String;

        cipher = self.enc(&plain, &key);
        assert_eq!(self.dec(&cipher, &key), plain)
    }
}


#[test]
fn test_cbc_128_pkcs7() {
    let test = Test::new(Mode::cbc);

    test.enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test.enc_dec("hello", "YELLOW SUBMARINE");
    test.enc_dec("a", "YELLOW SUBMARINE");
}


#[test]
fn test_ecb_128_pkcs7() {
    let test = Test::new(Mode::ecb);

    test.enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test.enc_dec("hello", "YELLOW SUBMARINE");
    test.enc_dec("a", "YELLOW SUBMARINE");
    //test.enc_dec("", "YELLOW SUBMARINE");
}


#[test]
fn test_ctr() {
    let test = Test::new(Mode::ctr);

    test.enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test.enc_dec("hello", "YELLOW SUBMARINE");
    test.enc_dec("a", "YELLOW SUBMARINE");
    test.enc_dec("", "YELLOW SUBMARINE");
}

