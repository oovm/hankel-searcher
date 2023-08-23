use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hs_types::ferguson_pair_at;
use hs_moments::{delta_moments, gamma_moments, zeta_moments};

fn bench_delta_ferguson(c: &mut Criterion) {
    let moments = delta_moments(80);
    c.bench_function("delta_ferguson_n25", |b| {
        b.iter(|| black_box(ferguson_pair_at(&moments, 25).unwrap()))
    });
}

fn bench_gamma_ferguson(c: &mut Criterion) {
    let moments = gamma_moments(80);
    c.bench_function("gamma_ferguson_n25", |b| {
        b.iter(|| black_box(ferguson_pair_at(&moments, 25).unwrap()))
    });
}

fn bench_zeta3_ferguson(c: &mut Criterion) {
    let moments = zeta_moments(3, 80);
    c.bench_function("zeta3_ferguson_n25", |b| {
        b.iter(|| black_box(ferguson_pair_at(&moments, 25).unwrap()))
    });
}

criterion_group!(
    benches,
    bench_delta_ferguson,
    bench_gamma_ferguson,
    bench_zeta3_ferguson
);
criterion_main!(benches);
