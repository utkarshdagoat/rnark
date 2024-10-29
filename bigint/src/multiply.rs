use std::{cmp::Ordering, u64};

use crate::BigUint;

pub(crate) const KARASTUBA_THRESHOLD: usize = 2;

impl BigUint {
    pub fn scalar_mult(&self, scalar: u64) -> BigUint {
        let mut coefficients = vec![];
        let mut carry: u128 = 0;

        for a in self.coefficients.iter() {
            let mul_result = (scalar as u128) * (*a as u128) + carry;
            coefficients.push(mul_result as u64 & u64::MAX);
            carry = mul_result >> u64::BITS;
        }
        if carry != 0 {
            coefficients.push(carry as u64);
        }
        BigUint { coefficients }
    }

    // multiply a bigint by β^j complexity O(n)
    pub fn shift_power(&self, j: usize) -> BigUint {
        let mut copy = self.clone().coefficients;
        let mut new_coff: Vec<u64> = vec![0; j]; // starting j coffi
        new_coff.append(&mut copy); // updated_coff[j] = prev_coff[0]
        BigUint {
            coefficients: new_coff,
        }
    } 

    pub fn shift_power_self(&mut self, j: usize) {
        let mut copy = self.clone().coefficients;
        let mut new_coff: Vec<u64> = vec![0; j]; // starting j coffi
        new_coff.append(&mut copy); // updated_coff[j] = prev_coff[0]
        self.coefficients = new_coff;
    }


    /// This algorithm is Base case / simple multiplication algorithm
    pub fn base_case_mult(a: &BigUint, b: &BigUint) -> BigUint {
        // c = A.b0
        let mut c = a.scalar_mult(b.coefficients[0]);
        for (j, bi) in b.coefficients.iter().skip(1).enumerate() {
            let temp = a.shift_power(j + 1).scalar_mult(*bi);
            // c = &c + &temp;
            c += &temp;
        }
        c
        
    }

    // In our model as β = 2⁶⁴ = T i.e the max value by the cofficients
    // Thus taking modulo   β^k  is same  as doing addition for the first k cofficients for the BigUint
    pub fn modulo_bases(&self, power: usize) -> BigUint {
        let mut coffs = vec![];
        for i in 0..power {
            coffs.push(self.coefficients[i]);
        }
        BigUint {
            coefficients: coffs,
        }
    }

    pub fn div_bases(&self, power: usize) -> BigUint {
        let mut coffs = vec![];
        for i in power..self.coefficients.len() {
            coffs.push(self.coefficients[i]);
        }
        BigUint {
            coefficients: coffs,
        }
    }

    // This is for balanced case i.e a.n = b.n
    pub fn karastuba(a: &BigUint, b: &BigUint) -> BigUint {
        let n = if a.coefficients.len() == b.coefficients.len() {
            a.coefficients.len()
        } else {
            panic!("Invalid inputs of unequal size");
        };
        if n <= KARASTUBA_THRESHOLD {
            return BigUint::base_case_mult(a, b);
        }
        let k = n / 2;
        let a0 = a.modulo_bases(k);
        let b0 = b.modulo_bases(k);
        let a1 = a.div_bases(k);
        let b1 = b.div_bases(k);
        let sa = match a0.partial_cmp(&a1) {
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            Some(Ordering::Less) => -1,
            None => panic!("Cannot compare!"),
        };
        let sb = match b0.partial_cmp(&b1) {
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            Some(Ordering::Less) => -1,
            None => panic!("Cannot compare!"),
        };
        //TODO: Add parralellisation for the follwoing computation
        // should we addC2 or subtract it
        let scalar = -sa * sb;
        let diff1 = &a0 - &a1;
        let diff2 = &b0 - &b1;

        let c0 = BigUint::karastuba(&a0, &b0);
        let c1 = BigUint::karastuba(&a1, &b1);
        let c2 = BigUint::karastuba(&diff1, &diff2);
        let mut c= &c0 + &c1;
        // C := C0 + (C0 + C1  + (− sAsB) C2)βk + C1β2k
        // C := C0 + (C0 + C1  + scalar*C2)βk + C1β2k
        if scalar > 0 {
            c += &c2;
        } else if scalar < 0 {
            c -= &c2;
        }
        c.shift_power_self(k);
        c += &c0;
        c += &c1.shift_power(2 * k);
        c
    }

    pub fn mult(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        if a.coefficients.len() <= 32 {
            return BigUint::base_case_mult(a, b);
        }
        BigUint::zero()
    }

}
