
use common::{util, err};


const dictionary_path = "/etc/dictionary-common/words";

pub fn filter(len: usize, prefix: &str) -> Result<Vec<String>, err::Error> {
    let wordlist = try!(util::read_file_to_str(&dictionary_path));
    let result = Vec::<String>::new();

    for word in wordlist.lines() {
        if word.len() == len && word.stars_with(&prefix) {
            result.push(word);
        }
    }
    Ok(result)
}

