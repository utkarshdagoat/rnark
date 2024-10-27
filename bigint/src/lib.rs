use std::f32::consts::E;

mod from;
mod into;
mod multiply;

/// BigUint is made up of u32's which are composable in little endian format i.e
/// lower values is coff[0]. The bases for our Bigint is β = 2^64
/// Thus a number A is representes as
/// A = sigma( a_i * β ^ (i) )
/// The struct contains the value of a_i's
/// n -> is the highest degree of β
/// coefficients[0] -> is the coff of  β^0 , coff[i] ->  β^1 and so on..
#[derive(Debug, Clone)]
pub struct BigUint {
    coefficients: Vec<u64>,
}
pub(crate) const BASES: u128 = 1 << (64); // 2^64

impl BigUint {
    // Big uint of zero
    pub fn zero() -> BigUint {
        BigUint {
            coefficients: vec![0],
        }
    }

    pub fn from_str_radix(s: &str, radix: u32) -> BigUint {
        let mut a = BigUint::zero();

        for char in s.chars() {
            let mut char_bigint = BigUint::from(char.to_digit(radix).unwrap());
            let mut temp = a.scalar_mult(radix.into());
            a = BigUint::add(&mut temp, &mut char_bigint);
        }
        a
    }

    /// add's two big int's and returns a new bigint
    pub fn add(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let mut d: u128 = 0; // A + B = d*BASE^n + C
        let n = if a.coefficients.len() > b.coefficients.len() {
            b.pad(a.coefficients.len());
            a.coefficients.len()
        } else {
            a.pad(b.coefficients.len());
            b.coefficients.len()
        };
        let mut coefficients: Vec<u64> = Vec::new();

        for i in 0..n {
            let s = (a.coefficients[i] as u128) + (b.coefficients[i] as u128) + d;
            d = s / BASES; // This number will also be less than 2^64
            coefficients.push((s % BASES) as u64);
        }
        if d != 0 {
            coefficients.push(d as u64);
        }
        BigUint { coefficients }
    }

    pub fn sub(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let mut d: u128 = 0; // A + B = d*BASE^n + C
        let n = if a.coefficients.len() > b.coefficients.len() {
            b.pad(a.coefficients.len());
            a.coefficients.len()
        } else {
            a.pad(b.coefficients.len());
            b.coefficients.len()
        };
        let mut coefficients: Vec<u64> = Vec::new();

        for i in 0..n {
            let s = (a.coefficients[i] as u128) - (b.coefficients[i] as u128) + d;
            d = s / BASES; // This number will also be less than 2^64
            coefficients.push((s % BASES) as u64);
        }
        if d != 0 {
            coefficients.push(d as u64);
        }
        BigUint { coefficients  }
    }

    // pads biguint to n with zeros
    pub fn pad(&mut self, n: usize) {
        for _ in self.coefficients.len()..n {
            self.coefficients.push(0);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{u128, u64};

    use super::BigUint;

    #[test]
    fn from_hex() {
        let value: u128 = 0x1A2B3C4D5E6F7890ABCDEF1234567890;
        let num = BigUint::from_str_radix(&value.to_string(), 10);
        assert_eq!(value, num.try_into().unwrap());
    }
    #[test]
    fn add_test_u64() {
        let a = u64::MAX / 3;
        let b = u64::MAX / 3;
        let mut a_bn = BigUint::from(a);
        let mut b_bn = BigUint::from(b);
        let sum = a + b;
        let sum_bn = BigUint::add(&mut a_bn, &mut b_bn);
        assert_eq!(sum, sum_bn.try_into().unwrap());
    }

    #[test]
    fn add_test_u128() {
        let a = u128::MAX / 2;
        let b = u128::MAX / 4;
        let mut a_bn = BigUint::from(a);
        let mut b_bn = BigUint::from(b);
        let sum = a + b;
        let sum_bn = BigUint::add(&mut a_bn, &mut b_bn);
        // assert_eq!(sum,3*(u128::MAX/4)); This failes cause of precision problems
        assert_eq!(sum, sum_bn.try_into().unwrap());
    }

    #[test]
    fn sub_test_u128() {
        let a = u128::MAX / 2;
        let b = u128::MAX / 4;
        let mut a_bn = BigUint::from(a);
        let mut b_bn = BigUint::from(b);
        let sub = a - b;
        let sub_bn = BigUint::sub(&mut a_bn, &mut b_bn);
        assert_eq!(sub, sub_bn.try_into().unwrap());
    }

    #[test]
    fn test_scalar_mult() {
        let a = u128::MAX / 2;
        let mut a_bn = BigUint::from(a);
        let scalar = 2;
        let scalar_result_bn = a_bn.scalar_mult(scalar);
        let scalar_result = (a as u128) * (scalar as u128);
        assert_eq!(scalar_result, scalar_result_bn.try_into().unwrap());
    }

    #[test]
    fn test_base_mult() {
        let mut a = BigUint::from_str_radix(
            "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
            16,
        );
        let mut b = BigUint::from_str_radix(
            "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
            16,
        );
        println!("a {:?}", a);
        println!("b {:?}", b);
        let result = BigUint::base_case_mult(&mut a, &mut b);
        // TODO: Assert when made to_str_radix function current testing id done through python :(
        println!("result {:?}", result);
    }
}
