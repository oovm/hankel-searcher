use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ops::Sub;
use ndarray::Array2;
use rand::{Rng, RngCore, SeedableRng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::SmallRng;
use crate::Cell;

#[derive(Debug)]
pub struct SquareSite<D = Uniform<u8>> {
    width: u32,
    lines: u32,
    next_id: u32,
    current: Vec<Cell>,
    groups: BTreeMap<u32, Vec<(u32, u32)>>,
    rng: SmallRng,
    dist: D,
}

pub enum MCState {
    Empty,
    Occupied,
    Complete,
}


impl<D> SquareSite<D> where D: Distribution<u8> {
    pub fn new(width: usize, dist: D) -> Self {
        let rng = SmallRng::from_rng(thread_rng()).expect("Failed to create rng");
        Self {
            width: width as u32,
            lines: 0,
            next_id: 1,
            current: vec![Cell::default(); width],
            groups: BTreeMap::default(),
            rng,
            dist,
        }
    }
    pub fn scan(&mut self) {
        let mut new = self.random_cells();
        let view = new.clone();
        // 每读一个像素, 有以下三种可能
        // 1. 和左边以及上面的格子颜色都不同
        //   - 这是一个新的块, 赋予新的 id
        // 2. 和左边或者上面的格子之一颜色相同
        //   - 赋予颜色相同的块的 id
        // 3. 左边, 上面, 此格, 皆同色
        //   - 合并左边和上面的区域 id
        for (column, cell) in new.iter_mut().enumerate() {
            if cell.get_id() == 0 {
                continue;
            }
            let left = self.left_cell(column, &view);
            let up = self.up_cell(column);
            if left.get_color() == up.get_color() {
                if cell.get_color() == up.get_color() {
                    let id = self.merge_groups(left.get_id(), up.get_id(), column).expect("Failed to merge groups");
                    cell.set_id(id)
                } else {
                    cell.set_id(self.create_groups(column));
                }
            } else {
                if cell.get_color() == up.get_color() {
                    self.groups.get_mut(&up.get_id()).expect("Failed to get group").push((self.lines, column as u32));
                    cell.set_id(up.get_id());
                } else if cell.get_color() == left.get_color() {
                    self.groups.get_mut(&left.get_id()).expect("Failed to get group").push((self.lines, column as u32));
                    cell.set_id(left.get_id());
                } else {
                    cell.set_id(self.create_groups(column));
                }
            }
        }
        self.current = new;
    }
    fn random_cells(&mut self) -> Vec<Cell> {
        let mut line = Vec::with_capacity(self.width as usize);
        for _ in 0..self.width {
            let color = self.dist.sample(&mut self.rng);
            line.push(Cell::new(color));
        }
        self.lines += 1;
        line
    }

    fn up_cell(&self, column: usize) -> Cell {
        unsafe {
            *self.current.get_unchecked(column)
        }
    }
    fn left_cell(&self, column: usize, view: &[Cell]) -> Cell {
        if column == 0 {
            Cell::new(0)
        } else {
            unsafe {
                *view.get_unchecked(column - 1)
            }
        }
    }
    fn merge_groups(&mut self, left_id: u32, up_id: u32, new: usize) -> Option<u32> {
        let left = self.groups.remove(&left_id)?;
        let up = self.groups.get_mut(&up_id)?;
        up.extend(left);
        up.push((self.lines, new as u32));
        Some(up_id)
    }

    fn create_groups(&mut self, column: usize) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        match self.groups.insert(id, vec![(self.lines, column as u32)]) {
            Some(s) => panic!("Duplicate group id `{}`: {:?}", id, s),
            None => {
                id
            }
        }
    }


    // pub fn dye(&mut self, colors: u8, dist: impl Distribution<u8>) {
    //     for cell in self.cells.iter_mut() {
    //         let next = dist.sample(&mut self.rng);
    //         cell.set_color(next % colors);
    //     }
    //     self.state = MCState::Occupied;
    // }
    // pub fn dye_uniform(&mut self, colors: u8) {
    //     self.dye(colors, Uniform::new(0, colors));
    // }
    // /// Write the same id to connected regions
    // pub fn distinguish(&mut self) {}
}

impl SquareSite<Uniform<u8>> {
    pub fn uniform(width: usize, color: u8) -> Self {
        Self::new(width, Uniform::<u8>::new(0, color))
    }
}