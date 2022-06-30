    use criterion::{black_box, criterion_group, criterion_main, Criterion};
use delta_encoding::{DeltaEncoder, DeltaEncoderExt};

criterion_group!(
    benches,
    bench_map_empty,
    bench_map_short,
    bench_map_long,
    bench_deltas_empty,
    bench_deltas_short,
    bench_deltas_long
);
criterion_main!(benches);

fn bench_map_empty(c: &mut Criterion) {
    c.bench_function("map empty", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            (0..1).map(|v| enc.encode(v)).for_each(|x: i64| {
                black_box(x);
            });
        })
    });
}

fn bench_map_short(c: &mut Criterion) {
    c.bench_function("map short", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            (0..1000).map(|v| enc.encode(v)).for_each(|x: i64| {
                black_box(x);
            });
        })
    });
}

fn bench_map_long(c: &mut Criterion) {
    c.bench_function("map long", |b| {
        b.iter(|| {
            let mut enc = DeltaEncoder::default();
            (0..100000).map(|v| enc.encode(v)).for_each(|x: i64| {
                black_box(x);
            });
        })
    });
}

fn bench_deltas_empty(c: &mut Criterion) {
    c.bench_function("deltas empty", |b| {
        b.iter(|| {
            (0..1).deltas().for_each(|x: i64| {
                black_box(x);
            })
        })
    });
}

fn bench_deltas_short(c: &mut Criterion) {
    c.bench_function("deltas short", |b| {
        b.iter(|| {
            (0..1000).deltas().for_each(|x: i64| {
                black_box(x);
            })
        })
    });
}

fn bench_deltas_long(c: &mut Criterion) {
    c.bench_function("deltas long", |b| {
        b.iter(|| {
            (0..100000).deltas().for_each(|x: i64| {
                black_box(x);
            })
        })
    });
}
