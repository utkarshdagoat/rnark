use crate::BigUint;
use std::cmp::Ordering;

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.coefficients.len() == other.coefficients.len() {
            for i in (0..self.coefficients.len()).rev() {
                if self.coefficients[i] > other.coefficients[i] {
                    return Some(Ordering::Greater);
                } else if self.coefficients[i] < other.coefficients[i] {
                    return Some(Ordering::Less);
                }
            }
            Some(Ordering::Equal)
        } else if self.coefficients.len() > other.coefficients.len() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}
