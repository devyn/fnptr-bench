use std::sync::atomic::AtomicBool;

use criterion::{criterion_group, criterion_main, Criterion};
use fnptr_bench::*;

fn bench_ptr(c: &mut Criterion) {
    c.bench_function(
        "ptr true",
        |b| {
            let val = AtomicBool::new(true);
            b.iter(|| one_two_ptr(&val)())
        }
    );
    c.bench_function(
        "ptr false",
        |b| {
            let val = AtomicBool::new(false);
            b.iter(|| one_two_ptr(&val)())
        }
    );
}

fn bench_cached_ptr(c: &mut Criterion) {
    c.bench_function(
        "cached ptr true",
        |b| {
            let val = AtomicBool::new(true);
            let cached = one_two_ptr(&val);
            b.iter(|| cached())
        }
    );
    c.bench_function(
        "cached ptr false",
        |b| {
            let val = AtomicBool::new(false);
            let cached = one_two_ptr(&val);
            b.iter(|| cached())
        }
    );
}

fn bench_direct(c: &mut Criterion) {
    c.bench_function(
        "direct true",
        |b| {
            let val = AtomicBool::new(true);
            b.iter(|| one_two_direct(&val))
        }
    );
    c.bench_function(
        "direct false",
        |b| {
            let val = AtomicBool::new(false);
            b.iter(|| one_two_direct(&val))
        }
    );
}

criterion_group!(benches,
    bench_ptr,
    bench_cached_ptr,
    bench_direct,
);
criterion_main!(benches);
