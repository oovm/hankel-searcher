use hs_types::ferguson_pair_at;
use hs_moments::{gamma_moment, zeta_moment, zeta_moments};

#[test]
fn debug_zeta2_n0() {
    let moments = zeta_moments(2, 6);
    println!("a1 = {:?}", moments.a(1));
    println!("a2 = {:?}", moments.a(2));
    println!("a3 = {:?}", moments.a(3));
    let pair = ferguson_pair_at(&moments, 0).unwrap();
    println!("P = {:?}", pair.p);
    println!("Q = {:?}", pair.q);
    println!("value = {:?}", pair.value().unwrap());
}

#[test]
fn debug_gamma_moments() {
    for n in 1..=4 {
        println!("n={} moment={:?}", n, gamma_moment(n));
    }
}

#[test]
fn debug_zeta_moment_formula() {
    println!("zeta(2) n=2 = {:?}", zeta_moment(2, 2));
}
