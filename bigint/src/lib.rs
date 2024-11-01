mod op_override;
mod comp;
mod from;
mod into;
mod multiply;
mod test;
mod division;

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
    //  zero big uint
    pub fn zero() -> BigUint {
        BigUint {
            coefficients: vec![0],
        }
    }

    pub fn from_str_radix(s: &str, radix: u32) -> BigUint {
        let mut a = BigUint::zero();

        for char in s.chars() {
            let char_bigint = BigUint::from(char.to_digit(radix).unwrap());
            let temp = a.scalar_mult(radix.into());
            a = &temp + &char_bigint;
        }
        a
    }

    /// add's two big int's and returns a new bigint
    pub fn add(x: &BigUint, y: &BigUint) -> BigUint {
        let (a, b) = if x > y { (x, y) } else { (y, x) };

        let mut d: u128 = 0; // A + B = d*BASE^n + C
        let mut coefficients: Vec<u64> = Vec::new();

        for (ai, bi) in a.coefficients.iter().zip(b.coefficients.iter()) {
            let s = (*ai as u128) + (*bi as u128) + d;
            d = s >> 64; // This number will also be less than 2^64
            coefficients.push(s as u64 & (u64::MAX));
        }

        for ai in a.coefficients.iter().skip(b.coefficients.len()) {
            let s = (*ai as u128) + d;
            d = s >> 64;
            coefficients.push(s as u64 & (u64::MAX));
        }

        // The carry is not zero therefore it is a new cofficient
        if d != 0 {
            coefficients.push(d as u64);
        }
        BigUint { coefficients }
    }

    // add's two big int's stores the result in the first big int
    pub fn add_acc(a: &mut BigUint, b: &BigUint) {
        let mut d: u128 = 0;


        for (i, ai) in a.coefficients.iter_mut().enumerate() {
            if i < b.coefficients.len() {
                let s = (*ai as u128) + (b.coefficients[i] as u128) + d;
                d = s >> 64; // This number will also be less than 2^64
                *ai = s as u64 & (u64::MAX);
            } else {
                if d == 0 {
                    break;
                }
                let s = (*ai as u128) + d;
                d = s >> 64;
                *ai = s as u64 & (u64::MAX);
            }
        }

        // if b > a
        if b.coefficients.len() > a.coefficients.len() {
            for bi in b.coefficients.iter().skip(a.coefficients.len()) {
                let s = (*bi as u128) + d;
                d = s >> 64;
                a.coefficients.push(s as u64 & (u64::MAX));
            }
        }

        // The carry is not zero therefore it is a new cofficient
        if d != 0 {
            a.coefficients.push(d as u64);
        }
    }

    /// Returns the absolute subtraction
    pub fn sub(x: &BigUint, y: &BigUint) -> BigUint {
        let (a, b) = if x > y { (x, y) } else { (y, x) };
        let mut rem = 0i128;
        const BASESI128: i128 = BASES as i128;

        let n = b.coefficients.len();
        let mut coefficients: Vec<u64> = Vec::new();

        for (ai, bi) in a.coefficients.iter().zip(b.coefficients.iter()) {
            rem += (*ai as i128) - (*bi as i128);
            if rem < 0 {
                coefficients.push((rem + BASESI128) as u64);
                rem = -1;
            } else {
                coefficients.push(rem as u64);
                rem = 0;
            }
        }

        for ai in a.coefficients.iter().skip(b.coefficients.len()) {
            if rem == 0 {
                let mut remaining_coffs: Vec<u64> = Vec::new();
                remaining_coffs.extend_from_slice(&a.coefficients[n..]);
                coefficients.append(&mut remaining_coffs);
                break;
            }
            rem += *ai as i128;
            if rem > BASESI128 {
                coefficients.push((rem - BASESI128) as u64);
                rem = 1;
            } else {
                coefficients.push(rem as u64);
                rem = 0;
            }
        }

        BigUint { coefficients }
    }

    // The caller has to make sure that a > b;
    fn sub_acc(a: &mut BigUint, b: &BigUint) {
        let mut rem = 0i128;
        const BASESI128: i128 = BASES as i128;

        for (i, ai) in a.coefficients.iter_mut().enumerate() {
            if i < b.coefficients.len() {
                rem += (*ai as i128) - (b.coefficients[i] as i128);
                if rem < 0 {
                    *ai = (rem + BASESI128) as u64;
                    rem = -1;
                } else {
                    *ai = rem as u64;
                    rem = 0;
                }
            } else {
                if rem == 0 {
                    break;
                }
                rem += *ai as i128;
                if rem > BASESI128 {
                    *ai = (rem - BASESI128) as u64;
                    rem = 1;
                } else {
                    *ai = rem as u64;
                    rem = 0;
                }
            }
        }
    }
}
