
use matasano::set2::ecbadmin;


#[test]
fn test_cryptopals_case() {
    let authbox = m!(ecbadmin::init_authbox());
    assert!(m!(ecbadmin::auth_as_admin(&authbox)));
}

