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


pub fn str_to_raw(input: &str) -> Result<Vec<u8>, err::Error> {
    let mut output: Vec<u8> = Vec::new();
    
    for c in input.chars() {
        output.push(c as u8);
    }
    Ok(output)
}
