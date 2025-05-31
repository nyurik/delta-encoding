use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use delta_encoding::{DeltaEncoder, DeltaEncoderExt};

criterion_group!(benches, bench_map, bench_iter);
criterion_main!(benches);

#[allow(clippy::cast_possible_wrap)]
fn bench_map(c: &mut Criterion) {
    let samples: &[u64] = &[1, 1000, 100_000];

    let mut group = c.benchmark_group("mapping");
    for count in samples {
        group.throughput(Throughput::Bytes(*count * 8));
        group.bench_function(format!("Encode {count} i64 values"), move |b| {
            b.iter(|| {
                let mut enc = DeltaEncoder::default();
                (0..(*count as i64))
                    .map(|v| enc.encode(v))
                    .for_each(|x: i64| {
                        black_box(x);
                    });
            });
        });
    }
    group.finish();
}

#[allow(clippy::cast_possible_wrap)]
fn bench_iter(c: &mut Criterion) {
    let samples: &[u64] = &[1, 1000, 100_000];

    let mut group = c.benchmark_group("iterator");
    for count in samples {
        group.throughput(Throughput::Bytes(*count * 8));
        group.bench_function(format!("Encode {count} i64 values"), move |b| {
            b.iter(|| {
                (0..(*count as i64)).deltas().for_each(|x: i64| {
                    black_box(x);
                });
            });
        });
    }
    group.finish();
}
