mod from;
mod into;

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
    n: usize,
}
pub(crate) const BASES: u128 = 1 << (64); // 2^64
impl BigUint {
    /// add's two big int's and returns a new bigint
    pub fn add(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let mut d: u128 = 0; // A + B = d*BASE^n + C
        let mut n = if a.n > b.n { a.n } else { b.n };
        a.pad(n);
        b.pad(n);
        let mut coefficients: Vec<u64> = Vec::new();

        for i in 0..n {
            let s = (a.coefficients[i] as u128) + (b.coefficients[i] as u128) + d;
            d = s / BASES; // This number will also be less than 2^64
            coefficients.push((s % BASES) as u64);
        }
        if d != 0 {
            coefficients.push(d as u64);
            n = n + 1;
        }
        BigUint { coefficients, n }
    }

    // pads biguint to n with zeros
    pub fn pad(&mut self, n: usize) {
        for _ in self.n..n {
            self.coefficients.push(0);
        }
    }
}

mod test {
    use std::u128;

    use super::BigUint;
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
        assert_eq!(sum, sum_bn.try_into().unwrap());
    }
}
