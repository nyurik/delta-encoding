use criterion::{criterion_group, criterion_main, Criterion};
use delta_encoding::DeltaEncoder;

criterion_group!(benches, bench_empty, bench_short, bench_long);
criterion_main!(benches);

fn bench_empty(c: &mut Criterion) {
    c.bench_function("empty", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..1).map(|v| enc.encode(v)).collect();
        })
    });
}

fn bench_short(c: &mut Criterion) {
    c.bench_function("short", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..1000).map(|v| enc.encode(v)).collect();
        })
    });
}

fn bench_long(c: &mut Criterion) {
    c.bench_function("long", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..100000).map(|v| enc.encode(v)).collect();
        })
    });
}
