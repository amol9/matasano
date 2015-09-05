
use common::err;


pub fn encode(params: &Vec<(&str, &str)>) -> Result<String, err::Error> {
    let mut result = String::new();

    for &(name, value) in params {
        ctry!(name.contains('&') || name.contains('='), format!("invalid char: &/= in name: {}", name));
        ctry!(value.contains('&') || value.contains('='), format!("invalid char: &/= in value: {}", value));

        result.push_str(&name);
        result.push('=');
        result.push_str(&value);
        result.push('&');
    }
    result.pop();
    Ok(result)
}


pub fn decode(param_string: &str) -> Result<Vec<(String, String)>, err::Error> {
    let mut result = Vec::<(String, String)>::new();

    for pair in param_string.split('&') {
        let mut nv = &mut pair.split('=');
        ctry!(nv.count() != 2, format!("invalid name-value pair: {}", pair));

        result.push((String::from(nv.next().unwrap()), 
                     String::from(nv.next().unwrap())));
    }
    Ok(result)
}

