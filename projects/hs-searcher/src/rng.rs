/// Deterministic pseudo-random utilities for reproducible sampling.
pub fn seed_to_u64(seed: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in seed.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn shuffle_indices(seed: &str, mut indices: Vec<usize>) -> Vec<usize> {
    if indices.len() <= 1 {
        return indices;
    }
    let mut state = seed_to_u64(seed);
    for i in (1..indices.len()).rev() {
        let j = (splitmix64(&mut state) as usize) % (i + 1);
        indices.swap(i, j);
    }
    indices
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
