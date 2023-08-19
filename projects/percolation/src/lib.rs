#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod cell;
mod errors;
mod square;

mod helpers;

pub use crate::{
    cell::Cell,
    errors::PercolationError,
    helpers::group::{MergeList, MergeListView},
    square::SquareSite,
};

// use wolfram_library_link::export;
//
// #[export]
// fn square_site(x: i64) -> i64 {
//     let mut ss = SquareSite::uniform(100, 2);
//     for i in ss.take(100) {
//         let sum = i.iter().sum::<usize>() as f32;
//         let mean = sum / i.len() as f32;
//         println!("{mean:>.2}: {i:?}");
//     }
// }
