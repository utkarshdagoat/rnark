use std::vec;

use crate::BigUint;

impl From<u32> for BigUint {
    fn from(value: u32) -> Self {
        let coff = vec![(value as u64)];
        BigUint {
            coefficients: coff,
        }
        
    }
}

impl From<u64> for BigUint {
    fn from(value: u64) -> Self {
        let coefficients = vec![value];
        BigUint { coefficients}
    }
}

impl From<u128> for BigUint {
    fn from(value: u128) -> Self {
        let bytes = value.to_be_bytes();
        let (high, low) = bytes.split_at(8);
        let coefficients = vec![
            u64::from_be_bytes(low.try_into().unwrap()),
            u64::from_be_bytes(high.try_into().unwrap()),
        ];
        BigUint {
            coefficients,
        }
    }
}
