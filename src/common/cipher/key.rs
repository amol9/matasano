extern crate rand;
use self::rand::Rng;

use common::err;


pub fn random(length: usize) -> Result<Vec<u8>, err::Error> {
    ctry!(length == 0, "invalid key length: 0");

    let mut rng = rand::thread_rng();
    Ok((0 .. length).map(|_| rng.gen::<u8>()).collect())
}
