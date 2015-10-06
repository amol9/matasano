// go through all the plain text strings:
//  collect the full words, like so:
//   have a buffer
//   go on appending to buffer one char at a time
//   when a whitespace or non-letter is encountered, clear the buffer
//   do this until the threshold of min col length
//
//  now, you should have a list of valid prefixes for unfinished strings
//  sort by ascending order of remaining undecrypted chars
//
//  A: do the following until all ciphers are done or no prefix yields a solution
//  for each prefix:
//    look up the dictionary for candidates which have same prefix and <= remaining length of cipher
//    string
//    for each candidate:
//      update keystream
//      with the updated keystream, update all the cipher's undecrypted suffixes
//      check each of these suffixes: their prefix + newly decrypted suffix should be valid words (or
//      parts of it: valid_partials())
//      if all are valid: great!!!
//        move ahead the fixed keystream length and update all prefix buffer strings
//        go to A
//      else:
//        exit loop and try the next candidate
//    next candidate
//  next prefix
//
//  return updated keystream
//
pub fn fix_keystream(ciphers: &Vec<Vec<u8>>, keystream: &Vec<u8>, min_col_len: usize) -> Result<Vec<u8>, err::Error> {
    let fixed_keystream = keystream.clone();
    let plains = xor_keystream(&ciphers, &keystream);

    let mut all_prefixes: Vec<Vec<u8>> = (0 .. plains.len()).map(|_| vec![]).collect();
 
    let plain_its: Vec<slice::Iter<'a, u8>> = plains.iter().map(|p| p.iter()).collect();
    let cipher_its: Vec<slice::Iter<'a, u8>> = ciphers.iter().map(|c| c.iter()).collect();
    let mut keystream_it = keystream.iter();

    let mut col_len = plains.len();

    while col_len >= min_col_len {
        col_len = 0;
        let mut prefix_it = prefixes.iter_mut();
        keystream_it.next();

        for plain_it in plain_its.iter_mut() {
            let mut prefix = prefix_it.next().unwrap();

            match plain_it.next() {
                Some(v) => {
                    if v is a non-word char {
                        prefix.clear();
                    } else {
                        prefix.push(v);
                    }
                    col_len += 1;
                },
                None    => {
                    prefix.clear();
                }
            };
        }
    }

    let mut cipher_suffixes: Vec<Vec<u8>>;          // undecrypted cipher suffixes
    let mut prefixes: Vec<Vec<u8>>;                 // corrsponding decrypted prefixes
    let all_prefixes_it = all_prefixes.iter(); 

    for cipher_it in cipher_its {
        let prefix = all_prefixes_it.next().unwrap();
        let remaining_cipher = cipher_it.collect();

        if remaining_cipher.len() > 0 {
            cipher_suffixes.push(remaining_cipher.clone());
            prefixes.push(prefix);
        }
    }

    let mut keystream_suffix: Vec<u8> = keystream_it.collect();  // keystream suffix which will be fixed

    try!(fix_keystream_suffix(&mut cipher_suffixes, &mut prefixes, &mut keystream_suffix));

    let truncate_len = fixed_keystream.len() - keystream_suffix.len();
    fixed_keystream.truncate(truncate_len);
    fixed_keystream.extend(&keystream_suffix);

    Ok(fixed_keystream)
}

fn is_valid_word_char(c: u8) -> bool {
    ((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || c == '\'' || c == '-')
}

fn fix_keystream_suffix(cipher_suffixes: &mut Vec<Vec<u8>>, prefixes: &mut Vec<Vec<u8>>, keystream_sufix: &mut Vec<u8>) ->
    Result<(), err:Error> {


}

const ascii_valid_chars: Vec<u8> = ascii::valid_chars()

// add a test
//
fn is_valid_partial(input: &Vec<u8>) -> bool {
    if input.any(|c| ascii_valid_chars.find(c).is_none()) {
        return false;
    }

    // break up on non-letter chars
    // for each part do:
    //   if part is a part of a valid word: return true
    //  
    //
    // start scanning from left
    //   accumulate letter chars until a non-letter char is encountered
    //   this string should be a suffix of a valid word
    //   
    //   let full_word = true
    //
    //   keep scanning
    //   accumulate letter chars until a non-letter char is encountered
    //   this string should be a full valid word
    //
    //   if string terminates w/o a non-letter
    //   this string should be a prefix of a valid word
    //
    //
}
