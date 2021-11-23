#[macro_use]
extern crate criterion;

use criterion::Criterion;
use secretbox::chacha20::implementation::*;
#[cfg(not(feature = "simd"))]
fn chacha20_impl_benchmark(c: &mut Criterion) {
    // Run once
    c.bench_function("chacha20 quarter_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            quarter_round(a, b, b, b)
        })
    });
    c.bench_function("chacha20 column_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            column_round([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b])
        })
    });
    c.bench_function("chacha20 diagonal_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            diagonal_round([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b])
        })
    });
    c.bench_function("chacha20 double_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            double_round([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b])
        })
    });

    c.bench_function("chacha20 x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            chacha20([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b])
        })
    });
}

#[cfg(feature = "simd")]
fn chacha20_impl_benchmark(c: &mut Criterion) {
    c.bench_function("chacha20 column_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            let (i1, i2, i3, i4) = prepare([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b]);
            column_round(i1, i2, i3, i4)
        })
    });
    c.bench_function("chacha20 diagonal_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            let (i1, i2, i3, i4) = prepare([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b]);
            diagonal_round(i1, i2, i3, i4)
        })
    });
    c.bench_function("chacha20 double_round x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            let (i1, i2, i3, i4) = prepare([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b]);
            double_round(i1, i2, i3, i4)
        })
    });
    c.bench_function("chacha20 x1", |b| {
        b.iter(|| {
            let a = criterion::black_box(1);
            let b = criterion::black_box(0);
            chacha20([a, b, b, b, a, b, b, b, a, b, b, b, a, b, b, b])
        })
    });
}

criterion_group!(benches, chacha20_impl_benchmark);
criterion_main!(benches);
