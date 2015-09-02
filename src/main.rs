use std::env;
use std::process;
use std::convert::AsRef;

extern crate matasano;
use self::matasano::set1;
use self::matasano::set2;
use self::matasano::common::challenge;


fn main() {
    let mut all_challenges = set1::challenges.iter().chain(set2::challenges.iter());

	let problem_no = match env::args().nth(1) {
		Some(v)	=> match v.parse::<u32>() {
            Ok(v)  => v,
            Err(e) => { println!("{}", e); process::exit(1); }
        },

		None	=> {
				println!("please specify a problem number to try");
				println!("or, help to list all sets");
				process::exit(1);
			   }
	};

    let challenge = match all_challenges.find(|ch| ch.no == problem_no) {
        Some(v) => v,
        None    => { println!("no such challenge"); process::exit(1); }
    };

    let r: i32 = (challenge.execute_fn)();

    process::exit(r);
}

