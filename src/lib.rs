use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[inline(never)]
pub fn one() -> i32 {
    1
}

#[inline(never)]
pub fn two() -> i32 {
    2
}

type NumFn = fn () -> i32;

pub fn one_two_ptr(b: &AtomicBool) -> NumFn {
    if b.load(Relaxed) {
        one
    } else {
        two
    }
}

pub fn one_two_direct(b: &AtomicBool) -> i32 {
    if b.load(Relaxed) {
        one()
    } else {
        two()
    }
}
