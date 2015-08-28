//macro to make a string
macro_rules! s {
	( $x : expr ) => ( String::from($x) );
}


//macro to check result
macro_rules! m {
    ( $x : expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    } );
}


//macro to check result and return specified value if error
macro_rules! mr {
    ( $x : expr, $ret: expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return $ret; }
    } );
}


