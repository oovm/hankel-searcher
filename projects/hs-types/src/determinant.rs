use num_rational::Ratio;
use num_traits::{One, Zero};

/// Build the square Hankel matrix `(m_{i+j})` for `0 <= i,j < size`.
pub fn hankel_matrix<T: Clone + Zero>(moments: &[T], shift: usize, size: usize) -> Vec<Vec<T>> {
    let mut matrix = Vec::with_capacity(size);
    for row in 0..size {
        let mut line = Vec::with_capacity(size);
        for col in 0..size {
            line.push(moments[shift + row + col].clone());
        }
        matrix.push(line);
    }
    matrix
}

/// Exact determinant via the Bareiss fraction-free elimination algorithm.
pub fn bareiss_det<T>(matrix: &[Vec<T>]) -> T
where
    T: Clone + Zero + One + std::ops::Neg<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    let n = matrix.len();
    if n == 0 {
        return T::one();
    }
    if n == 1 {
        return matrix[0][0].clone();
    }

    let mut work = matrix
        .iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pivot = T::one();
    let mut sign = T::one();
    for k in 0..n - 1 {
        if work[k][k].is_zero() {
            let swap = (k + 1..n).find(|row| !work[*row][k].is_zero());
            match swap {
                Some(row) => {
                    work.swap(k, row);
                    sign = -sign;
                }
                None => return T::zero(),
            }
        }

        for i in k + 1..n {
            for j in k + 1..n {
                let term = (work[i][j].clone() * work[k][k].clone() - work[i][k].clone() * work[k][j].clone())
                    / pivot.clone();
                work[i][j] = term;
            }
        }
        pivot = work[k][k].clone();
    }

    sign * work[n - 1][n - 1].clone()
}

/// Floating-point determinant via `faer` (pure Rust SIMD kernels, no BLAS sys).
pub fn f64_det(matrix: &[Vec<f64>]) -> f64 {
    let n = matrix.len();
    if n == 0 {
        return 1.0;
    }
    let m = faer::Mat::from_fn(n, n, |row, col| matrix[row][col]);
    m.determinant()
}

/// Floating-point Hankel determinant for asymptotic energy estimates.
pub fn hankel_det_f64(moments: &[f64], shift: usize, size: usize) -> f64 {
    let matrix = hankel_matrix(moments, shift, size);
    f64_det(&matrix)
}

/// Natural logarithm of `|det|` for log-energy diagnostics.
pub fn log_abs_hankel_det(moments: &[f64], shift: usize, size: usize) -> f64 {
    hankel_det_f64(moments, shift, size).abs().ln()
}

/// Estimate the quadratic decay rate `-log|H_n| / n^2` from a moment tail.
pub fn estimate_energy_rate(moments: &[f64], max_n: usize) -> Vec<(usize, f64)> {
    let mut rates = Vec::new();
    for n in 1..=max_n {
        let size = n + 1;
        if shifted_len(moments.len(), 0, size) {
            let log_det = log_abs_hankel_det(moments, 0, size);
            if log_det.is_finite() {
                rates.push((n, -log_det / (n as f64).powi(2)));
            }
        }
    }
    rates
}

fn shifted_len(total: usize, shift: usize, size: usize) -> bool {
    shift + 2 * size - 1 < total
}

/// Rational Hankel determinant.
pub fn hankel_det_rational(moments: &[Ratio<num_bigint::BigInt>], shift: usize, size: usize) -> Ratio<num_bigint::BigInt> {
    let matrix = hankel_matrix(moments, shift, size);
    bareiss_det(&matrix)
}
