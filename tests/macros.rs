
// macro to check result
macro_rules! r {
    ( $x : expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    } );
}

// macro to check result and return specified value if error
macro_rules! rr {
    ( $x : expr, $ret: expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return $ret; }
    } );
}

// macro to check if error is thrown
// on Ok  => false
// on Err => true
macro_rules! re {
    ( $x : expr ) => ( match $x {
        Ok(_)   => false,
        Err(e)  => { println!("{}", e); true }
    } );
}

// macro to check result and return None on error
macro_rules! rn {
    ( $x : expr ) => ( rr!( $x , None ) );
}

macro_rules! raw {
    ( $x : expr ) => ( r!( ascii::str_to_raw( $x ) ) );
}

macro_rules! rts {
    ( $x : expr ) => ( r!( ascii::raw_to_str( $x ) ) );
}

