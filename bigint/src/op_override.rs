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

impl ops::Mul<&BigUint> for &BigUint {
    type Output = BigUint;
    fn mul(self, rhs: &BigUint) -> Self::Output {
        BigUint::mult(self, rhs)
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

macro_rules! impl_mult_scalar {
    ($scalar: ty ) => {
        impl std::ops::Mul<$scalar> for &$crate::BigUint {
            type Output = $crate::BigUint;
            fn mul(self, rhs: $scalar) -> Self::Output {
                self.scalar_mult(rhs as u64)
            }
        }
    };
}

impl_mult_scalar!(i32);
impl_mult_scalar!(usize);
impl_mult_scalar!(u32);
impl_mult_scalar!(u64);