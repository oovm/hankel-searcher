use hs_benchmark::{
    delta_table, gamma_table, zeta2_table, zeta3_table, DELTA_REGRESSION_MAX_N,
    GAMMA_REGRESSION_MAX_N, ZETA5_DEFAULT_MAX_N, ZETA_REGRESSION_MAX_N,
};
use hs_types::{approximant_to_f64, ferguson_pair_at};
use hs_moments::{
    delta_moments, gamma_moments, zeta_moments, CATALAN_G, EULER_GOMPERTZ, EULER_MASCHERONI,
    ZETA_2, ZETA_3, ZETA_5,
};

const TOL: f64 = 5e-7;

fn assert_table_up_to(
    table: &[hs_benchmark::FergusonReference],
    max_n: usize,
    build_moments: impl Fn(usize) -> hs_types::RationalMomentSequence,
) {
    let rows: Vec<_> = table.iter().filter(|row| row.n <= max_n).collect();
    assert!(!rows.is_empty(), "no Ferguson rows with n <= {}", max_n);
    let table_max = rows.iter().map(|row| row.n).max().unwrap();
    let need = 2 * (table_max + 2);
    let moments = build_moments(need);
    for row in rows {
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

fn assert_table(
    table: &[hs_benchmark::FergusonReference],
    build_moments: impl Fn(usize) -> hs_types::RationalMomentSequence,
) {
    let max_n = table.iter().map(|row| row.n).max().unwrap();
    assert_table_up_to(table, max_n, build_moments);
}

#[test]
fn delta_matches_ferguson_table1() {
    assert_table_up_to(delta_table(), DELTA_REGRESSION_MAX_N, |count| delta_moments(count));
    let need = 2 * (DELTA_REGRESSION_MAX_N + 2);
    let moments = delta_moments(need);
    let value =
        approximant_to_f64(&ferguson_pair_at(&moments, DELTA_REGRESSION_MAX_N).unwrap()).unwrap();
    assert!(value < EULER_GOMPERTZ);
}

#[test]
#[ignore = "δ tight gap at n≥25 needs deep exact Hankel; run with cargo test -- --ignored"]
fn delta_tail_gap_deep() {
    let moments = delta_moments(64);
    let value = approximant_to_f64(&ferguson_pair_at(&moments, 29).unwrap()).unwrap();
    assert!(EULER_GOMPERTZ - value < 5e-9);
}

#[test]
fn gamma_matches_ferguson_table1() {
    assert_table_up_to(gamma_table(), GAMMA_REGRESSION_MAX_N, |count| gamma_moments(count));
    let need = 2 * (GAMMA_REGRESSION_MAX_N + 2);
    let moments = gamma_moments(need);
    let value =
        approximant_to_f64(&ferguson_pair_at(&moments, GAMMA_REGRESSION_MAX_N).unwrap()).unwrap();
    assert!(value < EULER_MASCHERONI);
}

#[test]
#[ignore = "γ exact Hankel at n≥20 is exponential; run with cargo test -- --ignored"]
fn gamma_matches_ferguson_table1_deep() {
    assert_table(gamma_table(), |count| gamma_moments(count));
    let moments = gamma_moments(60);
    let value = approximant_to_f64(&ferguson_pair_at(&moments, 25).unwrap()).unwrap();
    assert!(value < EULER_MASCHERONI);
}

#[test]
fn zeta2_matches_ferguson_table2() {
    assert_table_up_to(zeta2_table(), ZETA_REGRESSION_MAX_N, |count| zeta_moments(2, count));
    let need = 2 * (ZETA_REGRESSION_MAX_N + 2);
    let moments = zeta_moments(2, need);
    let value =
        approximant_to_f64(&ferguson_pair_at(&moments, ZETA_REGRESSION_MAX_N).unwrap()).unwrap();
    assert!(value < ZETA_2);
}

#[test]
fn zeta3_matches_ferguson_table2() {
    assert_table_up_to(zeta3_table(), ZETA_REGRESSION_MAX_N, |count| zeta_moments(3, count));
    let need = 2 * (ZETA_REGRESSION_MAX_N + 2);
    let moments = zeta_moments(3, need);
    let value =
        approximant_to_f64(&ferguson_pair_at(&moments, ZETA_REGRESSION_MAX_N).unwrap()).unwrap();
    assert!(value < ZETA_3);
    assert!(ZETA_3 - value < 5e-5);
}

#[test]
fn zeta5_ferguson_converges_below_constant() {
    let max_n = ZETA5_DEFAULT_MAX_N;
    let need = 2 * (max_n + 2);
    let moments = zeta_moments(5, need);
    for n in [0, 1, 5, 10, max_n] {
        let value = approximant_to_f64(&ferguson_pair_at(&moments, n).unwrap()).unwrap();
        assert!(value < ZETA_5, "n={}: got {:.12}", n, value);
    }
    let value = approximant_to_f64(&ferguson_pair_at(&moments, max_n).unwrap()).unwrap();
    assert!(ZETA_5 - value < 1e-7);
}

/// 奇分母核 `Σ C(n-1,i)(-1)^i/(2i+1)^2` 不满足 Ferguson 定理 1 的 `P_n/Q_n < G` 条件。
#[test]
fn catalan_odd_denominator_kernel_overshoots_constant() {
    use hs_moments::catalan_moments;
    let moments = catalan_moments(20);
    let value = approximant_to_f64(&ferguson_pair_at(&moments, 8).unwrap()).unwrap();
    assert!(value > CATALAN_G);
}

#[test]
fn catalan_beta_kernel_tracks_scaled_zeta_not_catalan() {
    use hs_moments::{catalan_beta_moments, zeta_moments, ZETA_2};
    let max_n = ZETA_REGRESSION_MAX_N;
    let need = 2 * (max_n + 2);
    let beta = catalan_beta_moments(need);
    let zeta = zeta_moments(2, need);
    let beta_value = approximant_to_f64(&ferguson_pair_at(&beta, max_n).unwrap()).unwrap();
    let zeta_value = approximant_to_f64(&ferguson_pair_at(&zeta, max_n).unwrap()).unwrap();
    assert!((beta_value - zeta_value / 4.0).abs() < 1e-6);
    assert!(beta_value < CATALAN_G);
    assert!((beta_value - ZETA_2 / 4.0).abs() < 5e-2);
}
