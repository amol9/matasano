
use matasano::set2::ecbadmin;


#[test]
fn test_cryptopals_case() {
    let authbox = r!(ecbadmin::init_authbox());
    assert!(r!(ecbadmin::auth_as_admin(&authbox)));
}

