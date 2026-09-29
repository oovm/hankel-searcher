//! Univariate polynomials over `Q` via exact interpolation.

use crate::bignum::{Integer, Rational};
use crate::bignum::{One, Zero};
use crate::determinant::bareiss_det_rational;
use crate::rational_linalg::rational_mat_inv;

/// `det(xB + A)` with ascending coefficients, matching Flint `det_poly` / `charpoly(-B^{-1}A) * det(B)`.
///
/// Evaluates the pencil determinant at `0..=n` integer points and interpolates. Each evaluation is
/// a single rational Bareiss determinant — far cheaper than polynomial elimination or `B^{-1}` +
/// matrix charpoly in naive arithmetic.
pub fn det_linear_pencil(a: &[Vec<Rational>], b: &[Vec<Rational>]) -> Result<Vec<Rational>, String> {
    let n = a.len();
    if n == 0 || a.iter().any(|row| row.len() != n) || b.len() != n || b.iter().any(|row| row.len() != n) {
        return Err("linear pencil requires matching square matrices".into());
    }

    let points: Vec<Rational> = (0..=n)
        .map(|index| Rational::from(Integer::from(index)))
        .collect();
    tracing::info!(matrix_size = n, samples = points.len(), "pencil determinant interpolation start");
    let mut values = Vec::with_capacity(points.len());
    for (sample, point) in points.iter().enumerate() {
        values.push(bareiss_det_rational(&pencil_at(a, b, point)));
        if sample == 0 || sample + 1 == points.len() || (sample + 1) % 5 == 0 {
            tracing::info!(
                phase = "pencil_det_sample",
                sample = sample + 1,
                samples = points.len(),
                pct = (sample + 1).saturating_mul(100) / points.len(),
            );
        }
    }
    tracing::info!("pencil determinant Vandermonde interpolation");
    vandermonde_interpolate_ascending(&points, &values)
}

fn pencil_at(a: &[Vec<Rational>], b: &[Vec<Rational>], point: &Rational) -> Vec<Vec<Rational>> {
    a.iter()
        .zip(b)
        .map(|(row_a, row_b)| {
            row_a
                .iter()
                .zip(row_b)
                .map(|(left, right)| left.clone() + point.clone() * right.clone())
                .collect()
        })
        .collect()
}

/// Solve `V c = y` for ascending monomial coefficients, where `V[i][j] = points[i]^j`.
fn vandermonde_interpolate_ascending(points: &[Rational], values: &[Rational]) -> Result<Vec<Rational>, String> {
    let n = points.len();
    if values.len() != n {
        return Err("interpolation length mismatch".into());
    }
    if n == 0 {
        return Ok(Vec::new());
    }

    let mut vandermonde = vec![vec![Rational::ZERO; n]; n];
    for row in 0..n {
        let mut power = Rational::ONE;
        for col in 0..n {
            vandermonde[row][col] = power.clone();
            power *= points[row].clone();
        }
    }
    solve_linear_system(&vandermonde, values)
}

fn solve_linear_system(matrix: &[Vec<Rational>], rhs: &[Rational]) -> Result<Vec<Rational>, String> {
    let inverse = rational_mat_inv(matrix).ok_or_else(|| "singular Vandermonde system".to_string())?;
    let n = rhs.len();
    let mut solution = vec![Rational::ZERO; n];
    for row in 0..n {
        for col in 0..n {
            solution[row] += inverse[row][col].clone() * rhs[col].clone();
        }
    }
    Ok(solution)
}
