use bigint::BigUint;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn karastuba_benchmark(c: &mut Criterion) {
    c.bench_function("karastuba mult", |b| {
        b.iter(|| {
            BigUint::karastuba(
                &mut BigUint::from_str_radix(
                    "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
                    16,
                ),
                &mut BigUint::from_str_radix(
                    "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
                    16,
                ),
            )
        })
    });
}

pub fn base_case_mult_benchmark(c: &mut Criterion) {
    c.bench_function("base case mult", |b| {
        b.iter(|| {
            BigUint::base_case_mult(
                &mut BigUint::from_str_radix(
                    "7FA1B2C3D4E5F60789ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE0",
                    16,
                ),
                &mut BigUint::from_str_radix(
                    "ABCDEF1234567890ABCDEF13579BDF2468ACE02468ACE02468ACE02468ACE0FF",
                    16,
                ),
            )
        })
    });
}

criterion_group!(benches, karastuba_benchmark , base_case_mult_benchmark);
criterion_main!(benches);
