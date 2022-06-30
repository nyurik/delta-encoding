use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use delta_encoding::{DeltaEncoder, DeltaEncoderExt};

criterion_group!(benches, bench_map, bench_iter);
criterion_main!(benches);

fn bench_map(c: &mut Criterion) {
    let samples: &[u64] = &[1, 1000, 100000];

    let mut group = c.benchmark_group("mapping");
    for count in samples.iter() {
        group.throughput(Throughput::Bytes(*count * 8));
        group.bench_function(format!("Encode {count} i64 values"), move |b| {
            b.iter(|| {
                let mut enc = DeltaEncoder::default();
                (0..(*count as i64))
                    .map(|v| enc.encode(v))
                    .for_each(|x: i64| {
                        black_box(x);
                    })
            })
        });
    }
    group.finish();
}

fn bench_iter(c: &mut Criterion) {
    let samples: &[u64] = &[1, 1000, 100000];

    let mut group = c.benchmark_group("iterator");
    for count in samples.iter() {
        group.throughput(Throughput::Bytes(*count * 8));
        group.bench_function(format!("Encode {count} i64 values"), move |b| {
            b.iter(|| {
                (0..(*count as i64)).deltas().for_each(|x: i64| {
                    black_box(x);
                })
            })
        });
    }
    group.finish();
}
