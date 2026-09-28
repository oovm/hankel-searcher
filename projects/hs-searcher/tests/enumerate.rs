use hs_checkpoint::{Observation, RationalData};
use hs_searcher::{SearchBudget, enumerate_indices};
use num_bigint::BigInt;
use num_rational::Ratio;

fn observation(n: usize, error_num: i32, error_den: u32) -> Observation {
    let approx = Ratio::new(BigInt::from(1), BigInt::from(2));
    let bound = Ratio::new(BigInt::from(error_num), BigInt::from(error_den));
    Observation {
        kind: "finite_approximation_error_upper".into(),
        n,
        series_terms: 64,
        approximant: RationalData::from_ratio(&approx),
        error_upper: RationalData::from_ratio(&bound),
    }
}

#[test]
fn enumerate_tracks_improvements_and_cursor() {
    let budget = SearchBudget::new(3);
    let report = enumerate_indices(4, None, &budget, |n| Ok(observation(n, 10 - n as i32, 1))).unwrap();
    assert_eq!(report.start_index, 4);
    assert_eq!(report.end_index, 7);
    assert_eq!(report.completed_steps, 3);
    assert_eq!(report.improvements, 3);
    assert!(report.bound_improved);
    assert_eq!(report.best.as_ref().map(|o| o.n), Some(6));
}

#[test]
fn enumerate_keeps_old_best_when_no_strict_improvement() {
    let initial = observation(2, 1, 10);
    let budget = SearchBudget::new(2);
    let report = enumerate_indices(5, Some(initial.clone()), &budget, |n| Ok(observation(n, 2, 10))).unwrap();
    assert_eq!(report.improvements, 0);
    assert!(!report.bound_improved);
    assert_eq!(report.best.as_ref().map(|o| o.n), Some(2));
}

#[test]
fn zero_step_budget_is_rejected() {
    let budget = SearchBudget::new(0);
    let err = enumerate_indices(0, None, &budget, |_| unreachable!()).unwrap_err();
    assert!(err.contains("max_steps"));
}
