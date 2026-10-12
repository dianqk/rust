#![crate_type = "lib"]

#[inline(never)]
pub fn indirect_deref_1(x: &(&i32,i32), out: fn(i32, i32), clobber: fn()) {
    out(*(*x).0, (*x).1);
    clobber();
    out(*(*x).0, (*x).1);
}


#[inline(never)]
pub fn direct_deref_2(x: (&i32,i32), out: fn(i32, i32), clobber: fn()) {
    let x = &x;
    out(*(*x).0, (*x).1);
    clobber();
    out(*(*x).0, (*x).1);
}
