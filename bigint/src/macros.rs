#[macro_export]
macro_rules! widen {
    ($x: expr) => {
        $x as u128
    };
    ($x: expr,$b: expr) => {
        $x as i128
    };
}

/// left shift an integer safely
#[macro_export]
macro_rules! safe_shift_right {
    ($x: expr , $bits: expr) => {{
        if $x > (1 << $bits) {
            $x >> $bits
        } else {
            0
        }
    }};
}
