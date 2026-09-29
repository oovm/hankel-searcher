use hs_checkpoint::zeta_series_bounds;
use hs_problems::{energy_eval_precision_bits, zeta5_log_s_k, zeta5_paper_params, zeta_integer_float};
use malachite::Float;
use malachite::base::num::conversion::traits::RoundingFrom;
use malachite::base::rounding_modes::RoundingMode::Nearest;

#[test]
fn log_s_k_matches_hankel_py_for_n1() {
    let params = zeta5_paper_params(1);
    let log_s = zeta5_log_s_k(&params);
    assert!((log_s - (-204.319)).abs() < 0.01);
}

#[test]
fn zeta5_float_head_reasonable_at_moderate_precision() {
    let z = zeta_integer_float(5, 512).expect("zeta5 float");
    let (head, _) = f64::rounding_from(&z, Nearest);
    assert!((head - 1.036_927_755_1).abs() < 1e-6);
}

#[test]
fn zeta5_float_lies_in_rigorous_series_enclosure() {
    let terms = 5_000usize;
    let (lower, upper) = zeta_series_bounds(5, terms).unwrap();
    let prec_bits = 512u64;
    let work_prec = prec_bits + 64;
    let value = zeta_integer_float(5, prec_bits).expect("zeta5 float");
    let (lower_f, _) = Float::from_rational_prec_ref(&lower, work_prec);
    let (upper_f, _) = Float::from_rational_prec_ref(&upper, work_prec);
    assert!(
        value >= lower_f && value <= upper_f,
        "zeta5={value} lower={lower_f} upper={upper_f}"
    );
}

#[test]
fn energy_eval_precision_requires_borwein_not_capped_series() {
    let prec = energy_eval_precision_bits(7597, 40);
    let required = 2f64.powf((prec as f64 + 1.0) / 4.0).ceil() as usize;
    assert!(required > 200_000, "n=1 energy must not use capped 200k direct sum");
}

#[test]
fn borwein_zeta5_head_at_moderate_precision() {
    let z = zeta_integer_float(5, 2_048).expect("borwein zeta5");
    let (head, _) = f64::rounding_from(&z, Nearest);
    assert!((head - 1.036_927_755_1).abs() < 1e-9);
}

#[test]
fn zeta5_float_agrees_across_high_precision_brackets() {
    let moderate = zeta_integer_float(5, 512).expect("zeta5 at 512");
    let high = zeta_integer_float(5, 1_024).expect("zeta5 at 1024");
    let (moderate_head, _) = f64::rounding_from(&moderate, Nearest);
    let (high_head, _) = f64::rounding_from(&high, Nearest);
    assert!((moderate_head - high_head).abs() < 1e-12);
}
