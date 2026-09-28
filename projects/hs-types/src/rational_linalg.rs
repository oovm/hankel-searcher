use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

type Rational = Ratio<BigInt>;

/// Matrix multiply for square rational matrices.
pub fn rational_mat_mul(left: &[Vec<Rational>], right: &[Vec<Rational>]) -> Vec<Vec<Rational>> {
    let n = left.len();
    debug_assert_eq!(left[0].len(), n);
    debug_assert_eq!(right.len(), n);
    debug_assert_eq!(right[0].len(), n);
    let mut out = vec![vec![Rational::zero(); n]; n];
    for row in 0..n {
        for col in 0..n {
            let mut acc = Rational::zero();
            for k in 0..n {
                acc += left[row][k].clone() * right[k][col].clone();
            }
            out[row][col] = acc;
        }
    }
    out
}

/// Invert a square rational matrix via Gauss-Jordan elimination.
pub fn rational_mat_inv(matrix: &[Vec<Rational>]) -> Option<Vec<Vec<Rational>>> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    let mut work = matrix
        .iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut inv = vec![vec![Rational::zero(); n]; n];
    for i in 0..n {
        inv[i][i] = Rational::one();
    }

    for col in 0..n {
        let pivot_row = (col..n).find(|row| !work[*row][col].is_zero())?;
        if pivot_row != col {
            work.swap(col, pivot_row);
            inv.swap(col, pivot_row);
        }
        let pivot = work[col][col].clone();
        for j in 0..n {
            work[col][j] = work[col][j].clone() / pivot.clone();
            inv[col][j] = inv[col][j].clone() / pivot.clone();
        }
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = work[row][col].clone();
            if factor.is_zero() {
                continue;
            }
            let pivot_row = work[col].clone();
            let inv_pivot_row = inv[col].clone();
            for j in 0..n {
                work[row][j] -= factor.clone() * pivot_row[j].clone();
                inv[row][j] -= factor.clone() * inv_pivot_row[j].clone();
            }
        }
    }
    Some(inv)
}

/// Characteristic polynomial `det(xI - matrix)` with ascending coefficients.
pub fn rational_charpoly(matrix: &[Vec<Rational>]) -> Vec<Rational> {
    let n = matrix.len();
    if n == 0 {
        return vec![Rational::one()];
    }
    debug_assert!(matrix.iter().all(|row| row.len() == n));

    let mut powers = vec![identity_matrix(n)];
    for index in 1..=n {
        powers.push(rational_mat_mul(matrix, &powers[index - 1]));
    }

    let mut coeffs = vec![Rational::zero(); n + 1];
    coeffs[n] = Rational::one();
    for k in 1..=n {
        let pk = matrix_trace(&powers[k]);
        let mut acc = pk;
        for j in 1..k {
            acc += coeffs[n - j].clone() * matrix_trace(&powers[k - j]);
        }
        coeffs[n - k] = -acc / Rational::from_integer(BigInt::from(k as i64));
    }
    coeffs
}

fn matrix_trace(matrix: &[Vec<Rational>]) -> Rational {
    (0..matrix.len()).fold(Rational::zero(), |acc, i| acc + matrix[i][i].clone())
}

fn identity_matrix(n: usize) -> Vec<Vec<Rational>> {
    let mut matrix = vec![vec![Rational::zero(); n]; n];
    for i in 0..n {
        matrix[i][i] = Rational::one();
    }
    matrix
}

/// Scale every coefficient of an ascending polynomial.
pub fn scale_polynomial(coeffs: &[Rational], factor: &Rational) -> Vec<Rational> {
    coeffs.iter().map(|coeff| coeff * factor).collect()
}
