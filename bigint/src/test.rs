#[cfg(test)]
mod test {
    use crate::BigUint;
    #[test]
    fn from_hex() {
        let value: u128 = 0x1A2B3C4D5E6F7890ABCDEF1234567890;
        let num = BigUint::from_str_radix(&value.to_string(), 10);
        assert_eq!(value, num.try_into().unwrap());
    }
    #[test]
    fn add_test_u64() {
        let a = u64::MAX / 3;
        let b = u64::MAX / 3;
        let mut a_bn = BigUint::from(a);
        let mut b_bn = BigUint::from(b);
        let sum = a + b;
        let sum_bn = BigUint::add(&mut a_bn, &mut b_bn);
        assert_eq!(sum, sum_bn.try_into().unwrap());
    }

    #[test]
    fn add_test_u128() {
        let a = u128::MAX / 2;
        let b = u128::MAX / 4;
        let mut a_bn = BigUint::from(a);
        let mut b_bn = BigUint::from(b);
        let sum = a + b;
        let sum_bn = BigUint::add(&mut a_bn, &mut b_bn);
        // assert_eq!(sum,3*(u128::MAX/4)); This failes cause of precision problems
        assert_eq!(sum, sum_bn.try_into().unwrap());
    }

    #[test]
    fn add_acc_test() {
        let mut a = BigUint::from_str_radix(
            "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
            16,
        );
        let b = BigUint::from_str_radix(
            "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
            16,
        );
        a += &b;
        let sum = BigUint::from_str_radix(
            "135438778702150141747206744445507229745568383754075343799015122984544803065311",
            10,
        );
        assert_eq!(a, sum);
    }

    #[test]
    fn sub_acc_test() {
        let mut a = BigUint::from_str_radix(
            "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
            16,
        );
        let b = BigUint::from_str_radix(
            "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
            16,
        );
        a -= &b;
        // calculated from python
        let result: BigUint = BigUint::from_str_radix(
            "19979922826221569069368642657004976767510873894317816736274411222541793506335",
            10,
        );
        assert_eq!(a, result);
    }

    #[test]
    fn sub_test_u128() {
        let a = u128::MAX / 2;
        let b = u128::MAX / 4;
        let mut a_bn = BigUint::from(a);
        let mut b_bn = BigUint::from(b);
        let sub = a - b;
        let sub_bn = BigUint::sub(&mut a_bn, &mut b_bn);
        assert_eq!(sub, sub_bn.try_into().unwrap());
    }

    #[test]
    fn test_sub_two_biguints() {
        let mut b = BigUint::from_str_radix(
            "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
            16,
        );
        let mut a = BigUint::from_str_radix(
            "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
            16,
        );

        let sub = BigUint::sub(&mut b, &mut a);
        // calculated from python
        let result: BigUint = BigUint::from_str_radix(
            "19979922826221569069368642657004976767510873894317816736274411222541793506335",
            10,
        );
        assert_eq!(sub, result);
    }

    #[test]
    fn test_scalar_mult() {
        let a = u128::MAX / 2;
        let a_bn = BigUint::from(a);
        let scalar = 2;
        let scalar_result_bn = a_bn.scalar_mult(scalar);
        let scalar_result = (a as u128) * (scalar as u128);
        assert_eq!(scalar_result, scalar_result_bn.try_into().unwrap());
    }

    #[test]
    fn test_base_mult() {
        let mut a = BigUint::from_str_radix(
            "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
            16,
        );
        let mut b = BigUint::from_str_radix(
            "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
            16,
        );
        let result = BigUint::base_case_mult(&mut a, &mut b);
        // done from python
        let actual_result = BigUint::from_str_radix(
            "4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624", 
            10
       );
        assert_eq!(actual_result, result);
    }

    #[test]
    fn test_karastuba() {
        let mut a = BigUint::from_str_radix(
            "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
            16,
        );
        let mut b = BigUint::from_str_radix(
            "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
            16,
        );
        let result = BigUint::karastuba(&mut a, &mut b);
        // done from python
        let actual_result = BigUint::from_str_radix(
            "4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624", 
            10
       );
        assert_eq!(actual_result, result);
    }

    #[test]
    fn test_odd_even_karastuba() {
        let a = BigUint::from_str_radix(
            "249166486039954038678241653028365438597414117400323598999",
            10,
        );
        let b = BigUint::from_str_radix("4062882539673212545835653973091204775", 10);
        let result = BigUint::odd_even_karastuba(&a, &b);

        let calculated_result = BigUint::from_str_radix(
            "1012334165603458524512240045393511714004426277361308599536572405543598559603493271800894020225",
        10
        );

        assert_eq!(result, calculated_result);
    }
}
