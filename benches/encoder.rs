use criterion::{criterion_group, criterion_main, Criterion};
use delta_encoding::{DeltaEncoder, DeltaEncoderExt};

criterion_group!(
    benches,
    bench_map_empty,
    bench_map_short,
    bench_map_long,
    bench_to_deltas_empty,
    bench_to_deltas_short,
    bench_to_deltas_long
);
criterion_main!(benches);

fn bench_map_empty(c: &mut Criterion) {
    c.bench_function("map empty", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..1).map(|v| enc.encode(v).unwrap()).collect();
        })
    });
}

fn bench_map_short(c: &mut Criterion) {
    c.bench_function("map short", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..1000).map(|v| enc.encode(v).unwrap()).collect();
        })
    });
}

fn bench_map_long(c: &mut Criterion) {
    c.bench_function("map long", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            let _: Vec<i64> = (0..100000).map(|v| enc.encode(v).unwrap()).collect();
        })
    });
}

fn bench_to_deltas_empty(c: &mut Criterion) {
    c.bench_function("to_deltas empty", |b| {
        b.iter(|| {
            let _: Vec<i64> = (0..1).to_deltas().collect();
        })
    });
}

fn bench_to_deltas_short(c: &mut Criterion) {
    c.bench_function("to_deltas short", |b| {
        b.iter(|| {
            let _: Vec<i64> = (0..1000).to_deltas().collect();
        })
    });
}

fn bench_to_deltas_long(c: &mut Criterion) {
    c.bench_function("to_deltas long", |b| {
        b.iter(|| {
            let _: Vec<i64> = (0..100000).to_deltas().collect();
        })
    });
}
