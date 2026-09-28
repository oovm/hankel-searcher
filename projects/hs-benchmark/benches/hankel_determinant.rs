//! Small-scale benchmarks for exact Hankel / pencil determinant routes (`hs-types` + Malachite).

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hs_types::{
    Integer, Rational, bareiss_det, bareiss_det_rational, det_linear_pencil, hankel_det_rational,
    hankel_matrix, rational_charpoly, rational_mat_inv, rational_mat_mul, scale_polynomial,
};
use malachite::base::num::basic::traits::{One, Zero};

fn q(n: i64, d: i64) -> Rational {
    Rational::from_integers(Integer::from(n), Integer::from(d))
}

/// Fibonacci-style moment tail for small Hankel benches.
fn fibonacci_moments(count: usize) -> Vec<Rational> {
    let mut values = vec![Rational::ZERO, Rational::ONE];
    while values.len() < count {
        let next = values[values.len() - 1].clone() + values[values.len() - 2].clone();
        values.push(next);
    }
    values
}

fn pencil_pair(size: usize) -> (Vec<Vec<Rational>>, Vec<Vec<Rational>>) {
    let moments = fibonacci_moments(2 * size);
    let a = hankel_matrix(&moments, 0, size);
    let b = hankel_matrix(&moments, 1, size);
    (a, b)
}

fn delta_via_charpoly(a: &[Vec<Rational>], b: &[Vec<Rational>]) -> Vec<Rational> {
    let det_b = bareiss_det(b);
    let b_inv = rational_mat_inv(b).expect("invertible");
    let neg_a = a
        .iter()
        .map(|row| row.iter().map(|entry| -entry.clone()).collect())
        .collect::<Vec<_>>();
    let c = rational_mat_mul(&b_inv, &neg_a);
    scale_polynomial(&rational_charpoly(&c), &det_b)
}

fn bench_bareiss_rational(c: &mut Criterion) {
    let mut group = c.benchmark_group("bareiss_rational");
    for size in [3usize, 5, 8] {
        let moments = fibonacci_moments(2 * size);
        let matrix = hankel_matrix(&moments, 0, size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &matrix, |b, matrix| {
            b.iter(|| black_box(bareiss_det_rational(matrix)));
        });
    }
    group.finish();
}

fn bench_bareiss_generic(c: &mut Criterion) {
    let mut group = c.benchmark_group("bareiss_generic_rational");
    for size in [3usize, 5, 8] {
        let moments = fibonacci_moments(2 * size);
        let matrix = hankel_matrix(&moments, 0, size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &matrix, |b, matrix| {
            b.iter(|| black_box(bareiss_det(matrix)));
        });
    }
    group.finish();
}

fn bench_hankel_det(c: &mut Criterion) {
    let mut group = c.benchmark_group("hankel_det_rational");
    for size in [3usize, 5, 8] {
        let moments = fibonacci_moments(2 * size + 1);
        group.bench_with_input(BenchmarkId::from_parameter(size), &moments, |b, moments| {
            b.iter(|| black_box(hankel_det_rational(moments, 0, size)));
        });
    }
    group.finish();
}

fn bench_pencil_vs_charpoly(c: &mut Criterion) {
    let mut pencil = c.benchmark_group("det_linear_pencil");
    let mut charpoly = c.benchmark_group("delta_via_charpoly");
    for size in [3usize, 5, 8] {
        let (a, b) = pencil_pair(size);
        pencil.bench_with_input(BenchmarkId::from_parameter(size), &(a, b), |bench, (a, b)| {
            bench.iter(|| black_box(det_linear_pencil(a, b).expect("pencil")));
        });
        charpoly.bench_with_input(BenchmarkId::from_parameter(size), &(a, b), |bench, (a, b)| {
            bench.iter(|| black_box(delta_via_charpoly(a, b)));
        });
    }
    pencil.finish();
    charpoly.finish();
}

fn bench_dense_pencil_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("pencil_point_det");
    for size in [3usize, 5, 8] {
        let (a, b) = pencil_pair(size);
        let point = q(2, 1);
        let matrix = a
            .iter()
            .zip(&b)
            .map(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b)
                    .map(|(left, right)| left.clone() + point.clone() * right.clone())
                    .collect()
            })
            .collect::<Vec<_>>();
        group.bench_with_input(BenchmarkId::from_parameter(size), &matrix, |b, matrix| {
            b.iter(|| black_box(bareiss_det_rational(matrix)));
        });
    }
    group.finish();
}

criterion_group!(
    hankel_determinant,
    bench_bareiss_rational,
    bench_bareiss_generic,
    bench_hankel_det,
    bench_pencil_vs_charpoly,
    bench_dense_pencil_eval
);
criterion_main!(hankel_determinant);
