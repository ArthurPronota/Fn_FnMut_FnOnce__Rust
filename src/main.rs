use std::mem::drop ;

fn main() {
    let s = String::from("abc") ;
    let f1 = || println!("{s}") ;   // Fn(), захват по &T
    f1() ;  // Out: abc

    // ---------------------------------------

    let mut v = vec![1, 2, 3, 4, 5] ;
    // f2 является mut
    let mut f2 = || {
            v.push(6) ; 
            println!("{v:?}") ;
        } ; // FnMut(), захват по &mut T
    f2() ; // Out: [1, 2, 3, 4, 5, 6]

    // ---------------------------------------

    let f3 = || {
            drop(s) ;
            println!("s has droped!") ;
        } ; // FnOnce(), потребляет s
    f3() ;  // Out: s has droped!

    /* use of moved value: `s`
    s ;
    */

    // ---------------------------------------

    let x = 2 ;
    let f4 = move || drop(x) ;  // Fn(), т.к. x: Copy несмотря на move
    f4() ;
    println!("x: {x}") ;    // Out: x: 2
}
