use crate::{Cell, MergeList};
use rand::{
    distributions::{Distribution, Uniform},
    prelude::SmallRng,
    thread_rng, SeedableRng,
};
use std::collections::BTreeMap;

/// Each grid is connected to the surrounding four grids, and the color obeys a given distribution
#[derive(Debug)]
pub struct SquareSite<D = Uniform<u8>> {
    width: u32,
    lines: u32,
    last: Vec<Cell>,
    groups: MergeList<(u32, u32)>,
    rng: SmallRng,
    dist: D,
}

pub enum MCState {
    Empty,
    Occupied,
    Complete,
}

impl<D> SquareSite<D>
where
    D: Distribution<u8>,
{
    pub fn new(width: usize, dist: D) -> Self {
        let rng = SmallRng::from_rng(thread_rng()).expect("Failed to create rng");
        Self { width: width as u32, lines: 0, last: vec![Cell::default(); width], groups: Default::default(), rng, dist }
    }
    pub fn scan(&mut self, lines: usize) {
        for _ in 0..lines {
            self.scan_line();
        }
    }
    pub fn scan_line(&mut self) {
        let mut new = self.random_cells();
        for column in 0..self.width {
            let up = self.up_cell(column as usize);
            let left = self.left_cell(column as usize, &new);
            let cell = new.get_mut(column as usize).expect("Failed to get cell");
            if cell.is_white() {
                continue;
            }
            if up.same_color(&left) {
                // 左边, 上面, 此格, 皆同色
                // - 合并左边和上面的区域 id
                if cell.same_color(&up) {
                    println!("情况 1");
                    cell.set_id(self.merge_group(left, up, column))
                }
                else {
                    println!("情况 3.1");
                    cell.set_id(self.insert_new_group(column));
                }
            }
            else {
                // 和左边或者上面的格子之一颜色相同
                // 赋予颜色相同的块的 id
                if cell.same_color(&up) {
                    println!("情况 2.1");
                    cell.set_id(self.insert_group(up, column));
                }
                else if cell.same_color(&left) {
                    // println!("情况 2.2");
                    cell.set_id(self.insert_group(left, column));
                }
                // 和左边以及上面的格子颜色都不同
                // 这是一个新的块, 赋予新的 id
                else {
                    println!("情况 3.2");
                    cell.set_id(self.insert_new_group(column));
                }
            }
            println!("{}:{}: {:?}", self.lines, column, cell);
        }
        self.last = new;
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
        unsafe { *self.last.get_unchecked(column) }
    }
    fn left_cell(&self, column: usize, view: &[Cell]) -> Cell {
        if column == 0 { Cell::new(0) } else { unsafe { *view.get_unchecked(column - 1) } }
    }
    fn merge_group(&mut self, left: Cell, up: Cell, column: u32) -> u32 {
        self.try_merge_group(left.get_id(), up.get_id(), column).expect("Failed to merge groups")
    }
    fn try_merge_group(&mut self, left_id: u32, up_id: u32, column: u32) -> Option<u32> {
        match self.groups.merge_group(left_id, up_id) {
            Some(_) => {}
            None => {
                unreachable!()
                // panic!("Failed to merge group {:?} and {:?}", up_id, left_id)
            }
        };

        // update new cell
        up.push((self.lines, column));
        // update record
        for cell in self.last.iter_mut() {
            if cell.get_id() == left_id {
                cell.set_id(up_id);
            }
        }
        Some(up_id)
    }
    fn insert_group(&mut self, old: Cell, column: u32) -> u32 {
        let id = old.get_id();
        match self.groups.get_mut(&id) {
            Some(s) => s.push((self.lines, column)),
            None => {
                unreachable!()
                // panic!("Failed to get left group {:?}\n{:?}", id, self.groups)
            }
        }
        id
    }

    fn insert_new_group(&mut self, column: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        match self.groups.insert(id, vec![(self.lines, column)]) {
            Some(_) => {
                unreachable!()
                // panic!("duplicate group id `{}`: {:?}", id, s)
            }
            None => id,
        }
    }
}

impl SquareSite<Uniform<u8>> {
    pub fn uniform(width: usize, color: u8) -> Self {
        Self::new(width, Uniform::<u8>::new(0, color))
    }
}
