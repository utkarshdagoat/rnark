// noramlize B
// most signification word is >= beta/2

use crate::BigUint;
const BASESBY2: u64 = 1 << 63; // beta/ 2 = 2^63
impl BigUint {
    pub fn noramlize(&mut self) -> Option<(BigUint, u64)> {
        let last_term = self.coefficients.last().unwrap();
        match *last_term < BASESBY2 {
            true => {
                let k = (BASESBY2 / last_term) + 1 as u64; // k less than beta
                println!("k: {}", k);
                Some((self.scalar_mult(k), k))
            }
            false => None,
        }
    }



    pub fn noramlize_with_given_k(&mut self, k: u64) {
        self.scalar_mult_self(k);
    }

    pub fn unnormalize(&mut self, k: u64) {
        for coff in self.coefficients.iter_mut() {
            *coff /= k;
        }
        // remove the last cofficient
        if self.coefficients.last().unwrap() == &0 {
            self.coefficients.pop();
        }
    }
}

#[test]
fn normalize() {
    let mut a = BigUint::from_str_radix(
        "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468",
        16,
    );
    println!("{:?}", a.coefficients);

    let (a_norm, k) = a.noramlize().unwrap();
    println!("{:?}", a_norm.coefficients);
    println!("{:?}", k);
}
