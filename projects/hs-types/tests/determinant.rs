use hs_types::{bareiss_det, f64_det, hankel_matrix};
use num_rational::Ratio;
use num_traits::Zero;

#[test]
fn bareiss_matches_small_integer_matrix() {
    let matrix = vec![vec![1, 2], vec![3, 4]];
    let det = bareiss_det(&matrix);
    assert_eq!(det, -2);
}

#[test]
fn f64_det_matches_bareiss_on_small_matrix() {
    let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let bareiss = bareiss_det(&matrix);
    let faer = f64_det(&matrix);
    assert!((bareiss - faer).abs() < 1e-12);
}

#[test]
fn hankel_det_of_moment_tail() {
    let moments = vec![
        Ratio::from_integer(0),
        Ratio::from_integer(1),
        Ratio::from_integer(2),
        Ratio::from_integer(5),
        Ratio::from_integer(16),
    ];
    let matrix = hankel_matrix(&moments, 0, 2);
    let det = bareiss_det(&matrix);
    assert!(!det.is_zero());
}
