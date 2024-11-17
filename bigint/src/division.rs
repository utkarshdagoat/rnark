use std::ops::Shr;

use crate::{widen, BigUint, BASES};
impl BigUint {
    // THis is my implementation for knuth's algorithm it has taken inspiration from the c algorithm in https://github.com/hcs0/Hackers-Delight/blob/master/divmnu64.c.txt
    pub fn divrem(a: &Self, b: &Self) -> (Self, Self) {
        let m = a.coefficients.len();
        let n = b.coefficients.len();
        let mut q = vec![0; m - n + 1]; //  quotient
        let mut r = vec![0; n]; // remainder

        if m < n {
            return (BigUint::zero(), a.clone());
        }

        if n == 0 {
            panic!("Biguint:Divide by Zero");
        }

        if n == 1 {
            let mut k = 0;
            for j in (0..m).rev() {
                q[j] =
                    ((k * BASES + widen!(a.coefficients[j])) / (widen!(b.coefficients[0]))) as u64;
                k = (k * BASES) - widen!(q[j]) * widen!(b.coefficients[0]);
            }
            r[0] = k as u64;
            return (BigUint::new(q.as_slice()), BigUint::new(r.as_slice()));
        }

        let s = b.coefficients[n - 1].leading_zeros();

        let mut vn = vec![0; n];
        let mut un = vec![0; m + 1];
        if s != 0 {
            for i in (1..n).rev() {
                vn[i] = (widen!(b.coefficients[i] << s) | widen!(b.coefficients[i - 1] >> (64 - s)))
                    as u64;
            }
            vn[0] = widen!(b.coefficients[0] << s) as u64;

            un[m] = a.coefficients[m - 1] >> (64 - s);
            for i in (1..m).rev() {
                un[i] = a.coefficients[i] << s | a.coefficients[i - 1] >> (64 - s) as u64;
            }
            un[0] = a.coefficients[0] << s;
        } else {
            vn.copy_from_slice(&b.coefficients);
            un[..m].copy_from_slice(&a.coefficients);
        }

        for j in (0..=m - n).rev() {
            let mut qhat = widen!(un[j + n]) * BASES + widen!(un[j + n - 1]);

            qhat /= widen!(vn[n - 1]);
            let mut rhat =
                (widen!(un[j + n]) * BASES + widen!(un[j + n - 1])) - qhat * widen!(vn[n - 1]);

            while qhat >= BASES || qhat * widen!(vn[n - 2]) > BASES * rhat + widen!(un[j + n - 2]) {
                qhat -= 1;
                rhat += widen!(vn[n - 1]);
                if rhat >= BASES {
                    break;
                }
            }

            let mut k: i128 = 0;
            let mut t: i128;
            for i in 0..n {
                let p = widen!(vn[i], true) * widen!(qhat, true);
                t = widen!(un[i + j], true) - k - widen!(p & 0xFFFFFFFFFFFFFFFF, true);
                un[i + j] = t as u64;

                k = p.shr(64) - t.shr(64);
            }
            t = widen!(un[j + n], true) - k;
            let last_term = t;
            q[j] = qhat as u64;

            if t < 0 {
                q[j] -= 1;
                k = 0;
                for i in 0..n {
                    t = widen!(un[i + j], true) + widen!(vn[i], true) + k;
                    un[i + j] = t as u64;
                    k = t >> 64;
                }
                un[j + n] = (last_term + k) as u64;
            } else {
                un[j + n] = t as u64;
            }
        }

        if s > 0 {
            for i in 0..n - 1 {
                r[i] = (un[i] >> s) | (un[i + 1] << (64 - s));
            }
            r[n - 1] = un[n - 1] >> s;
        } else {
            r.copy_from_slice(&un[..n]);
        }

        //remove leading zeros if any
        if let Some(last_non_zero) = q.iter().rposition(|&x| x != 0) {
            q.truncate(last_non_zero+1);
        }

        // as r can be zero we will also check None case
        match r.iter().rposition(|&x| x != 0) {
            Some(index) => r.truncate(index+1),
            None => r = vec![0],
        }

        (BigUint::new(&q), BigUint::new(&r))
    }
}

#[test]
fn test_base_case_divrem() {
    let a = BigUint::from_str_radix(
            "4486116365047057285615619245805733533662098016812356305731015669708311065831995465137397594684684578328578381687188591572342160574912716102443372011598624", 
            10
    );
    let b = BigUint::from_str_radix(
        "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
        16,
    );

    let calc_result = BigUint::from_str_radix("57729427937964286338919050894251126489028754929878763531370355881001504779488", 10);
    let (q,r) = BigUint::divrem(&a, &b);

    assert_eq!(r,BigUint::zero());
    assert_eq!(q,calc_result);
}
