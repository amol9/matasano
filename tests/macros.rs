
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


// macro to check if error is thrown
// on Ok  => return false
// on Err => return true
macro_rules! me {
    ( $x : expr ) => ( match $x {
        Ok(v)   => false,
        Err(e)  => { println!("{}", e); true }
    } );
}


macro_rules! rn {
    ( $x : expr ) => ( mr!( $x , None ) );
}


macro_rules! r {
    ( $x : expr ) => ( m!( ascii::str_to_raw( $x ) ) );
}


macro_rules! raw {
    ( $x : expr ) => ( m!( ascii::str_to_raw( $x ) ) );
}


macro_rules! rts {
    ( $x : expr ) => ( m!( ascii::raw_to_str( $x ) ) );
}

