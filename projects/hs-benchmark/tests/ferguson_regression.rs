use hs_benchmark::{delta_table, gamma_table, zeta2_table, zeta3_table};
use hs_types::{approximant_to_f64, ferguson_pair_at};
use hs_moments::{
    delta_moments, gamma_moments, zeta_moments, CATALAN_G, EULER_GOMPERTZ, EULER_MASCHERONI,
    ZETA_2, ZETA_3,
};

const TOL: f64 = 5e-7;

fn assert_table(
    table: &[hs_benchmark::FergusonReference],
    build_moments: impl Fn(usize) -> hs_types::RationalMomentSequence,
) {
    let max_n = table.iter().map(|row| row.n).max().unwrap();
    let need = 2 * (max_n + 2);
    let moments = build_moments(need);
    for row in table {
        let approx = ferguson_pair_at(&moments, row.n).expect("ferguson pair");
        let value = approximant_to_f64(&approx).expect("f64 value");
        assert!(
            (value - row.value).abs() < TOL,
            "n={}: got {:.12}, expected {:.12}",
            row.n,
            value,
            row.value
        );
    }
}

#[test]
fn delta_matches_ferguson_table1() {
    assert_table(delta_table(), |count| delta_moments(count));
    let moments = delta_moments(60);
    let last = ferguson_pair_at(&moments, 25).unwrap();
    let value = approximant_to_f64(&last).unwrap();
    assert!(value < EULER_GOMPERTZ);
    assert!(EULER_GOMPERTZ - value < 5e-9);
}

#[test]
fn gamma_matches_ferguson_table1() {
    assert_table(gamma_table(), |count| gamma_moments(count));
    let moments = gamma_moments(60);
    let last = ferguson_pair_at(&moments, 25).unwrap();
    let value = approximant_to_f64(&last).unwrap();
    assert!(value < EULER_MASCHERONI);
}

#[test]
fn zeta2_matches_ferguson_table2() {
    assert_table(zeta2_table(), |count| zeta_moments(2, count));
    let moments = zeta_moments(2, 60);
    let last = ferguson_pair_at(&moments, 25).unwrap();
    let value = approximant_to_f64(&last).unwrap();
    assert!(value < ZETA_2);
}

#[test]
fn zeta3_matches_ferguson_table2() {
    assert_table(zeta3_table(), |count| zeta_moments(3, count));
    let moments = zeta_moments(3, 60);
    let last = ferguson_pair_at(&moments, 25).unwrap();
    let value = approximant_to_f64(&last).unwrap();
    assert!(value < ZETA_3);
    assert!(ZETA_3 - value < 2e-6);
}

#[test]
fn catalan_experimental_converges_below_constant() {
    use hs_moments::catalan_moments;
    let moments = catalan_moments(40);
    let approx = ferguson_pair_at(&moments, 8).unwrap();
    let value = approximant_to_f64(&approx).unwrap();
    assert!(value > 0.0);
    assert!(value < CATALAN_G);
}
