extern crate matasano;

use self::matasano::set1::detectaesecb as dae;


#[test]
fn test_cryptopals_case() {
    let result = m!(dae::detect_from_list("data/8.txt"));

    assert_eq!(result.len(), 1);
    assert_eq!(result[0],
        "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708\
        649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a9\
        3eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b2\
        9933f2c123c58386b06fba186a");
}

