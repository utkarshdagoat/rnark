mod from;
mod into;

/// BigUint is made up of u32's which are composable in little endian format i.e
/// lower values. The bases for our Bigint is u32::MAX + 1 -> β = 2^32
/// Thus a number A is representes as
/// A = sigma( a_i * β ^ (i) )
/// The struct contains the value of a_i's
/// n -> is the highest degree of β
/// coefficients[0] -> is the coff of  β^0 , coff[i] ->  β^1 and so on..
#[derive(Debug,Clone)]
pub struct BigUint {
    coefficients: Vec<u64>,
    n: usize,
}
pub(crate) const BASES: u64 = 1 << (32); // 2^32 10000000000...0

impl BigUint {
    /// add's two big int's and returns a new bigint
    fn add(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let mut d: u64 = 0; // A + B = d*BASE^n + C
        let mut n = if a.n > b.n { a.n } else { b.n };
        a.pad(n);
        b.pad(n);
        let mut coefficients: Vec<u64> = Vec::new();

        for i in 0..n {
            let s = a.coefficients[i] + b.coefficients[i] + d;
            d = s / BASES;
            coefficients.push(s % BASES);
        }
        if d != 0 {
            coefficients.push(d);
            n = n + 1;
        }
        BigUint { coefficients, n }
    }

    // pads biguint to n with zeros
    fn pad(&mut self, n: usize) {
        for _ in self.n..n {
            self.coefficients.push(0);
        }
    }
}

// generic traits 
pub trait IntoBigUint {
    fn into_biguint(self) -> BigUint;
}

mod test {

    use crate::BigUint;
    // u32 and u64 test's are trivial
    fn test_from_u128(){
        let value = u128::MAX;
        let biguint = BigUint::from(value);
    }
}