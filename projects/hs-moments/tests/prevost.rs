use hs_moments::{prevost_weight, WeightFamily};

#[test]
fn prevost_zeta5_weight_is_positive_on_unit_interval() {
    let weight = prevost_weight(WeightFamily::Zeta5, 1);
    for step in 1..=20 {
        let t = step as f64 / 20.0;
        assert!(weight.eval(t) > 0.0, "t={}", t);
    }
    assert_eq!(weight.pole(), 16.0);
}
