use crate::BigUint;
use std::cmp::Ordering;

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.coefficients.len().cmp(&other.coefficients.len()) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                for i in (0..self.coefficients.len()).rev() {
                    match self.coefficients[i].cmp(&other.coefficients[i]) {
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Equal => continue,
                    }
                }
                Some(Ordering::Equal)
            }
        }
    }
}
