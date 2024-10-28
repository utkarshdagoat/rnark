use std::f32::consts::E;

mod comp;
mod from;
mod into;
mod multiply;
mod test;
/// BigUint is made up of u32's which are composable in little endian format i.e
/// lower values is coff[0]. The bases for our Bigint is β = 2^64
/// Thus a number A is representes as
/// A = sigma( a_i * β ^ (i) )
/// The struct contains the value of a_i's
/// n -> is the highest degree of β
/// coefficients[0] -> is the coff of  β^0 , coff[i] ->  β^1 and so on..
#[derive(Debug, Clone, PartialEq, Eq)]
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
        let mut swapped = false;
        // a is always bigger than b
        if b.coefficients.len() > a.coefficients.len() {
            std::mem::swap(a, b);
            swapped = false;
        }
        let n = b.coefficients.len();
        let mut coefficients: Vec<u64> = Vec::new();

        for i in 0..n {
            let s = (a.coefficients[i] as u128) + (b.coefficients[i] as u128) + d;
            d = s / BASES; // This number will also be less than 2^64
            coefficients.push((s % BASES) as u64);
        }
        // add the remaning to vec
        for i in n..a.coefficients.len() {
            let s = (a.coefficients[i] as u128) + d;
            d = s / BASES; // This number will also be less than 2^64
            coefficients.push((s % BASES) as u64);
        }
        // The carry is not zero therefore it is a new cofficient
        if d != 0 {
            coefficients.push(d as u64);
        }
        if swapped {
            std::mem::swap(a, b);
        }
        BigUint { coefficients }
    }


    /// Returns the absolute subtraction
    pub fn sub(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let mut rem = 0i128;
        let mut swapped = false;
        const BASESI128: i128 = BASES as i128;
        if b > a {
            std::mem::swap(a, b);
            swapped = true;
        }
        let n = b.coefficients.len();
        let mut coefficients: Vec<u64> = Vec::new();

        for i in 0..n {
            rem += (a.coefficients[i] as i128) - (b.coefficients[i] as i128);
            if rem < 0 {
                let ci = (rem + BASESI128) as u64;
                coefficients.push(ci);
                rem = -1;
            } else {
                coefficients.push(rem as u64);
                rem = 0;
            }
        }
        for i in n..a.coefficients.len() {
            if rem == 0 {
                let mut remaining_coffs: Vec<u64> = Vec::new();
                remaining_coffs.extend_from_slice(&a.coefficients[n..]);
                coefficients.append(&mut remaining_coffs);
                break;
            }
            rem += a.coefficients[i] as i128;
            if rem > BASESI128 {
                a.coefficients[i] = (rem - BASESI128) as u64;
                rem = 1;
            } else {
                a.coefficients[i] = rem as u64;
                rem = 0;
            }
        }
        if swapped {
            std::mem::swap(a, b);
        }
        BigUint { coefficients }
    }

    // pads biguint to n with zeros
    pub fn pad(&mut self, n: usize) {
        for _ in self.coefficients.len()..n {
            self.coefficients.push(0);
        }
    }
}
