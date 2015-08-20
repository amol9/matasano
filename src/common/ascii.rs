use std::char;

use common::err;


pub fn raw_to_string(raw: &Vec<u8>) -> Result<String, err::Error> {
    let mut result = String::new();

    for byte in raw {
        result.push(char::from_u32(*byte as u32).unwrap());
    }

    Ok(result)
    //mkerr!("sdkfjsd")
}
