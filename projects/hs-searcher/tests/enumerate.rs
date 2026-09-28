use hs_checkpoint::{Observation, RationalData};
use hs_searcher::{SearchBudget, SearchStrategy, enumerate_indices, local_indices, sample_indices};
use num_bigint::BigInt;
use num_rational::Ratio;
use std::sync::Mutex;

fn observation(n: usize, error_num: i32, error_den: u32) -> Observation {
    let approx = Ratio::new(BigInt::from(1), BigInt::from(2));
    let bound = Ratio::new(BigInt::from(error_num), BigInt::from(error_den));
    Observation {
        kind: "finite_approximation_error_upper".into(),
        n,
        shift: None,
        series_terms: 64,
        approximant: RationalData::from_ratio(&approx),
        error_upper: RationalData::from_ratio(&bound),
    }
}

#[test]
fn enumerate_tracks_improvements_and_cursor() {
    let budget = SearchBudget::new(3);
    let report = enumerate_indices(4, None, &budget, 1, |n| Ok(observation(n, 10 - n as i32, 1))).unwrap();
    assert_eq!(report.start_index, 4);
    assert_eq!(report.end_index, 7);
    assert_eq!(report.completed_steps, 3);
    assert_eq!(report.improvements, 3);
    assert!(report.bound_improved);
    assert_eq!(report.best.as_ref().map(|o| o.n), Some(6));
}

#[test]
fn local_prefers_indices_near_center() {
    let budget = SearchBudget::new(3);
    let seen = Mutex::new(Vec::new());
    let report = local_indices(3, 4, None, &budget, 1, |n| {
        seen.lock().expect("seen lock").push(n);
        Ok(observation(n, 100 - n as i32, 1))
    })
    .unwrap();
    assert_eq!(seen.lock().expect("seen lock").clone(), vec![4, 5, 6]);
    assert_eq!(report.end_index, 7);
}

#[test]
fn sample_is_seed_stable() {
    let budget = SearchBudget::new(4);
    let first = sample_indices("zeta-3", 10, None, &budget, 1, |n| Ok(observation(n, 100, 1))).unwrap();
    let second = sample_indices("zeta-3", 10, None, &budget, 1, |n| Ok(observation(n, 100, 1))).unwrap();
    assert_eq!(first.completed_steps, 4);
    assert_eq!(second.completed_steps, 4);
    assert_eq!(first.end_index, 14);
}

#[test]
fn search_strategy_parse_rejects_unknown() {
    assert!(SearchStrategy::parse("enumerate").is_ok());
    assert!(SearchStrategy::parse("quantum").is_err());
}

#[test]
fn parallel_enumerate_matches_sequential_merge() {
    let budget = SearchBudget::new(4);
    let sequential =
        enumerate_indices(2, None, &budget, 1, |n| Ok(observation(n, 20 - n as i32, 1))).unwrap();
    let parallel =
        enumerate_indices(2, None, &budget, 2, |n| Ok(observation(n, 20 - n as i32, 1))).unwrap();
    assert_eq!(parallel.end_index, sequential.end_index);
    assert_eq!(parallel.best.as_ref().map(|o| o.n), sequential.best.as_ref().map(|o| o.n));
    assert_eq!(parallel.improvements, sequential.improvements);
}
#[test]
fn zero_step_budget_is_rejected() {
    let budget = SearchBudget::new(0);
    let err = enumerate_indices(0, None, &budget, 1, |_| unreachable!()).unwrap_err();
    assert!(err.contains("max_steps"));
}
