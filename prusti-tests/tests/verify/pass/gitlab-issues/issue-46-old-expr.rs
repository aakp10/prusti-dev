/// Issue #46 "Field access of an old expressions"

use prusti_contracts::*;

struct S {
    f: i32
}

#[requires(x.f == 123)]
#[ensures(old(x.f) == 123)]
#[ensures(old(x).f == 456)]
fn test(x: &mut S) {
    x.f = 456;
}

fn main() {}
