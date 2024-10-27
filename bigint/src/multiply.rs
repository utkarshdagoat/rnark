use crate::{BigUint, BASES};

pub(crate) const KARASTUBA_THRESHOLD: usize = 2;

impl BigUint {
    pub fn scalar_mult(&mut self, scalar: u64) -> BigUint {
        let mut n = self.coefficients.len();
        let mut coefficients = vec![];
        let mut carry: u128 = 0;
        for i in 0..n {
            let mul_result = (scalar as u128) * (self.coefficients[i] as u128) + carry;
            coefficients.push((mul_result % BASES) as u64);
            carry = mul_result / BASES; // represetns the excess multiplication factor
        }
        if carry != 0 {
            coefficients.push(carry as u64);
            n = n + 1;
        }
        BigUint { coefficients}
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

    /// This algorithm is Base case for the karastuba's algorithm
    pub fn base_case_mult(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        // c = A.b0
        let mut C = a.scalar_mult(b.coefficients[0]);
        let n = b.coefficients.len();
        for j in 1..n {
            // our goal is to do this C ← C + β^j (A · bj )
            // so to do this I first shift the power of A as
            // A = ∑n−1 (aiβ^i)  -> multplying by  β^j is just
            // shifting the vector of cofficients as ai will be the cofficient for
            // (i+j)th base
            let mut temp = a.shift_power(j).scalar_mult(b.coefficients[j]);
            C = BigUint::add(&mut C, &mut temp)
        }
        C
    }

    // In our model as β = 2⁶⁴ = T i.e the max value by the cofficients
    // Thus taking modulo   β^k  is same  as doing addition for the first k cofficients for the BigUint
    pub fn modulo_bases(&self, power: usize) -> BigUint {
        //  As k < 2^64 ( a very resonable assumption)
        // summation of cofficients cannot increase a 128
        // Therefore we can directly sum up the cofficietns and  convert the u128 into BigUint
        let mut sum: u128 = 0;
        for i in 0..power {
            sum += self.coefficients[i] as u128;
        }
        BigUint::from(sum)
    }

    pub fn div_bases(&self,power: usize) -> BigUint {
        let mut coffs = vec![];
        for i in power..self.coefficients.len() {
            coffs.push(self.coefficients[i]);
        }
        BigUint {
            coefficients:coffs
        }
    }

    // This is for balanced case i.e a.n = b.n
    pub fn karastuba(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let n = if a.coefficients.len() == b.coefficients.len() {
            a.coefficients.len()
        } else {
            panic!("Invalid inputs of unequal size");
        };
        if n < KARASTUBA_THRESHOLD {
            return BigUint::base_case_mult(a, b);
        }
        BigUint::zero()
    }
}
