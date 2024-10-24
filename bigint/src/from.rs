use crate::BigUint;

impl From<u32> for BigUint {
    fn from(value: u32) -> Self {
        let coff = vec![(value as u64)];
        BigUint {
            coefficients: coff,
            n: 1,
        }
    }
}

impl From<u64> for BigUint {
    fn from(value: u64) -> Self {
        let coefficients = vec![value];
        BigUint { coefficients, n: 1 }
    }
}

impl From<u128> for BigUint {
    fn from(value: u128) -> Self {
        let lower_bits = value as u64; // Rust automatically removes the upper 64 bits
        let upper_bits = (value >> u64::BITS) as u64;
        let coefficients = vec![lower_bits, upper_bits];
        BigUint { coefficients, n: 2 }
    }
}
