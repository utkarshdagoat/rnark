use crate::{safe_shift_right, widen, BASES};

pub fn umul(a: u64, b: u64) -> (u64, u64) {
    let p = widen!(a) * widen!(b);

    if p > (1 << 64) {
        let hi = (p >> 64) as u64;
        let lo = p as u64;
        (hi, lo)
    } else {
        (0, p as u64)
    }
}

pub fn umullo(a: u64, b: u64) -> u64 {
    let p = widen!(a) * widen!(b);
    (p & BASES) as u64
}

pub fn umulhi(a: u64, b: u64) -> u64 {
    let p = widen!(a) * widen!(b);
    if p > BASES {
        (p >> 64) as u64
    } else {
        0
    }
}


