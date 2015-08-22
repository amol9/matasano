extern crate matasano;
use self::matasano::common::hex;


fn print_raw(raw: Vec<u8>) {
	for i in raw {
		print!("{} ", i);
	}
	println!("");
}


macro_rules! _gen {
	( $x:expr, $y:expr, $z: expr, $fun: expr ) => (
		match $fun($x) {
			Ok(v)	=> assert_eq!(v, $y),
			Err(e)	=> assert!($z),
		};
	);
}


macro_rules! _htr {
	( $x:expr, $y:expr, $z: expr ) => ( _gen!($x, $y, $z, hex::hex_to_raw));
}

macro_rules! htr {
	( $x: expr, $y: expr ) => ( _htr!($x, $y, false) );
}

macro_rules! htr_err {
	( $x: expr, $y: expr ) => ( _htr!($x, $y, true) );
}


macro_rules! _rth {
	( $x:expr, $y:expr, $z: expr ) => ( _gen!($x, $y, $z, hex::raw_to_hex::<hex::lower>));
}

macro_rules! rth {
	( $x: expr, $y: expr ) => ( _rth!($x, $y, false) );
}

macro_rules! rth_err {
	( $x: expr, $y: expr ) => ( _rth!($x, $y, true) );
}


macro_rules! s {
	( $x : expr ) => ( String::from($x) );
}


#[test]
fn test_hex_to_raw() {
	htr!(&s!("00"), vec![0]);
	htr!(&s!("ff"), vec![255]);
	htr!(&s!("aa"), vec![170]);
	htr!(&s!("aaff"), vec![170, 255]);

	htr_err!(&s!("0"), vec![]);
	htr_err!(&s!("000"), vec![]);
	htr_err!(&s!("0s"), vec![]);
}


#[test]
fn test_raw_to_hex() {
	rth!(&vec![0], s!("00"));
	rth!(&vec![255], s!("ff"));
	rth!(&vec![170], s!("aa"));
	rth!(&vec![170, 255], s!("aaff"));

	//rth_err!(&vec![345], s!(""));
	//rth_err!(&vec![256], s!(""));
	//rth_err!(&vec![0x0], s!(""));
}	
