use std::str::FromStr;

use bigint::BigUint;

use num_bigint::BigUint as num_biguint;

fn main() {
    let a = BigUint::from_str_radix("4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624", 10);
    let b = BigUint::from_str_radix(
        "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
        16,
    );

    println!("b {:?}", b);
    let result = BigUint::divrem(&a, &b);
    println!("{:?}", result.0);
    println!("{:?}", result.1);

    let a_bn = num_biguint::from_str("4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624").unwrap();
    let b_bn = num_biguint::from_str(
        "77709350764185855408287693551256103256539628824196580267644767103543298285823",
    )
    .unwrap();
    let div = &a_bn / &b_bn;

    let rem = &a_bn % &b_bn;

    println!("{:?}",div.to_str_radix(10));
    println!("{:?}",rem.to_str_radix(10));

}
