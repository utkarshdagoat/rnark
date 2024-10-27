use crate::BigUint;

// return the u32 version of coff[0]
impl TryInto<u32> for BigUint {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        if self.coefficients.len() > 1 {
            Err(())
        } else {
            Ok(self.coefficients[0] as u32)
        }
    }
}

impl TryInto<u64> for BigUint {
    type Error = ();
    fn try_into(self) -> Result<u64, Self::Error> {
        if self.coefficients.len() > 1 {
            Err(())
        } else {
            Ok(self.coefficients[0])
        }
    }
}

impl TryInto<u128> for BigUint {
    type Error = ();
    fn try_into(self) -> Result<u128, Self::Error> {
        if self.coefficients.len() > 2 {
            Err(())
        } else {
            let lower = self.coefficients[0] as u128;
            let upper = (self.coefficients[1] as u128) << u64::BITS;
            Ok(upper | lower)
        }
    }
}
