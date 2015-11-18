
use matasano::set1::aesdecrypt as ad;


#[test]
fn test_cryptopals_case() {
    let plain = r!(ad::decrypt_from_file("data/7.txt", "YELLOW SUBMARINE"));

    let exp_prefix: &str = "I'm back and I'm ringin' the bell \n\
                            A rockin' on the mike while the fly girls yell \n\
                            In ecstasy in the back of me \n\
                            Well that's my DJ Deshay cuttin' all them Z's \n";

    let exp_suffix: &str = "Play that funky music \n";

    assert!(plain.starts_with(exp_prefix));
    assert!(plain.ends_with(exp_suffix));

    println!("{}", plain);
}
