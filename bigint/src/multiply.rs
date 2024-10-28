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
        let mut coffs= vec![];
        for i in 0..power {
            coffs.push(self.coefficients[i]);
        }
        BigUint {
            coefficients:coffs
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
    pub fn karastuba(a: &mut BigUint, b: &mut BigUint) -> BigUint {
        let n = if a.coefficients.len() == b.coefficients.len() {
            a.coefficients.len()
        } else {
            panic!("Invalid inputs of unequal size");
        };
        if n <= KARASTUBA_THRESHOLD {
            return BigUint::base_case_mult(a, b);
        }
        let k = n / 2;
        let mut a0 = a.modulo_bases(k);
        let mut b0 = b.modulo_bases(k);
        let mut a1 = a.div_bases(k);
        let mut b1 = b.div_bases(k);
        let sa = if a0 == a1 {
            0
        } else if a0 > a1 {
            1
        } else {
            -1
        };
        let sb = if b0 == b1 {
            0
        } else if b0 > b1 {
            1
        } else {
            -1
        };
        // should we addC2 or subtract it
        let scalar = -sa * sb;
        let mut diff1 = BigUint::sub(&mut a0, &mut a1);
        let mut diff2 = BigUint::sub(&mut b0, &mut b1);

        let mut c0 = BigUint::karastuba(&mut a0, &mut b0);
        let mut c1 = BigUint::karastuba(&mut a1, &mut b1);
        let mut c2 = BigUint::karastuba(&mut diff1, &mut diff2);

        // C := C0 + (C0 + C1  + (− sAsB) C2)βk + C1β2k
        // C := C0 + (C0 + C1  + scalar*C2)βk + C1β2k

        let mut temp = BigUint::add(&mut c0, &mut c1);
        if scalar > 0 {
            temp = BigUint::add(&mut temp, &mut c2).shift_power(k);
        } else if scalar < 0 {
            temp = BigUint::sub(&mut temp, &mut c2).shift_power(k);
        }
        temp = BigUint::add(&mut temp, &mut c0);
        return BigUint::add(&mut c1.shift_power(2 * k), &mut temp);
    }
}
