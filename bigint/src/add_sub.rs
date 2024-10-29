use crate::BigUint;
use std::ops::{self, AddAssign};

impl ops::Add<&BigUint> for &BigUint {
    type Output = BigUint;
    fn add(self, rhs: &BigUint) -> Self::Output {
        BigUint::add(self, rhs)
    }
}

impl ops::Sub<&BigUint> for &BigUint {
    type Output = BigUint;
    fn sub(self, rhs: &BigUint) -> Self::Output {
        BigUint::sub(self, rhs)
    }
}

impl AddAssign<&BigUint> for BigUint {
    fn add_assign(&mut self, rhs: &BigUint) {
        BigUint::add_acc(self, rhs);
    }
}


impl ops::SubAssign<&BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: &BigUint) {
        BigUint::sub_acc(self, rhs);
    }
}
