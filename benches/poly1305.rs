#[macro_use]
extern crate criterion;

use criterion::Criterion;
use secretbox::poly1305::Poly1305;

fn poly1305_benchmark(c: &mut Criterion) {
    c.bench_function("16 byte", |b| {
        b.iter(|| {
            let a = criterion::black_box(0x1234);
            let b = criterion::black_box(0x4321);
            let c = criterion::black_box(0u8);
            let data = [c; 16];
            let mut enc = Poly1305::new(a, b);
            enc.hash(&data[..])
        })
    });
    c.bench_function("1 KiB", |b| {
        b.iter(|| {
            let a = criterion::black_box(0x1234);
            let b = criterion::black_box(0x4321);
            let c = criterion::black_box(0u8);
            let data = [c; 1024];
            let mut enc = Poly1305::new(a, b);
            enc.hash(&data[..])
        })
    });
    c.bench_function("64 KiB", |b| {
        b.iter(|| {
            let a = criterion::black_box(0x1234);
            let b = criterion::black_box(0x4321);
            let c = criterion::black_box(0u8);
            let data = [c; 65536];
            let mut enc = Poly1305::new(a, b);
            enc.hash(&data[..])
        })
    });
}

criterion_group!(benches, poly1305_benchmark);
criterion_main!(benches);
