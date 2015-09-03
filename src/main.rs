use std::env;
use std::process;
use std::convert::AsRef;

extern crate matasano;
use self::matasano::set1;
use self::matasano::set2;
use self::matasano::common::{challenge, charfreq};


fn main() {
    let command = match env::args().nth(1) {
        Some(v) => v,
        None    => { print_help(); process::exit(1); }
    };

    let ret = match command.as_ref() {
        "help"      => print_help(),
        "list"      => print_challenge_list(),
        "charfreq"  => charfreq::i_generate_base_frequency_file(),
        _           => execute_challenge(&command)
    };

    process::exit(ret);
}


fn execute_challenge(problem_no_str: &str) -> i32 {
    let mut all_challenges = set1::challenges.iter().chain(set2::challenges.iter());

	let problem_no = match problem_no_str.parse::<u32>() {
            Ok(v)  => v,
            Err(e) => { println!("{}", e); process::exit(1); }
    };

	let challenge = match all_challenges.find(|ch| ch.no == problem_no) {
        Some(v) => v,
        None    => { println!("no such challenge"); process::exit(1); }
    };

    (challenge.execute_fn)()
}


fn print_challenge_list() -> i32 {
    print_set(&set1::challenges, 1);
    print_set(&set2::challenges, 2);
    0
}


fn print_set(set: &[&challenge::Info], no: u32) {
    println!("Set-{}", no);
    for ch in set {
        println!("{:02}. {}", ch.no, ch.title);
    }
    println!("");
}


fn print_help() -> i32 {
    println!("usage: command [challenge_number | list | charfreq [sample_text_file]]\n\
                            challenge number:   to execute a challenge,\n\
                            list:               to print a list of all challenges\n\
                            charfreq:           to generate character frequency file needed for some challenges");
    0
}

