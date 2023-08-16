use std::collections::HashSet;
use ndarray::Array2;
use rand::{Rng, RngCore, SeedableRng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::SmallRng;
use crate::Cell;

pub struct SquareSite<D = Uniform<u8>> where D: Distribution<u8> {
    width: usize,
    current: Vec<Cell>,
    rng: SmallRng,
    dist: D,
}

pub enum MCState {
    Empty,
    Occupied,
    Complete,
}


impl SquareSite {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            state: MCState::Empty,
            cells: Array2::default((width, height)),
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        }
    }
    pub fn dye(&mut self, colors: u8, dist: impl Distribution<u8>) {
        for cell in self.cells.iter_mut() {
            let next = dist.sample(&mut self.rng);
            cell.set_color(next % colors);
        }
        self.state = MCState::Occupied;
    }
    pub fn dye_uniform(&mut self, colors: u8) {
        self.dye(colors, Uniform::new(0, colors));
    }
    /// Write the same id to connected regions
    pub fn distinguish(&mut self) {}
}