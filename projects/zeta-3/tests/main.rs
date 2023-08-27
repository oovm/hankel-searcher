use zeta_3::ferguson_approximant;

#[test]
fn ready() {
    let approx = ferguson_approximant(0, 8).unwrap();
    assert!(approx.index == 0);
}
