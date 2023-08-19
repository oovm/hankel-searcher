use crate::{Cell, MergeList};
use rand::{
    distributions::{Distribution, Uniform},
    prelude::SmallRng,
    thread_rng, SeedableRng,
};

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

impl<'i, D> Iterator for &'i mut SquareSite<D>
where
    D: Distribution<u8>,
{
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_line();
        Some(self.groups)
    }
}

impl<D> SquareSite<D>
where
    D: Distribution<u8>,
{
    /// Create a new square site with given width and distribution
    pub fn new(width: usize, dist: D) -> Self {
        let rng = SmallRng::from_rng(thread_rng()).expect("Failed to create rng");
        Self { width: width as u32, lines: 0, last: vec![Cell::default(); width], groups: Default::default(), rng, dist }
    }
    /// Scan lines with given distribution
    pub fn scan(&mut self, lines: usize) {
        for _ in 0..lines {
            self.scan_line();
        }
    }
    /// Scan one line with given distribution
    pub fn scan_line(&mut self) {
        let mut new = self.random_cells();
        for column in 0..self.width {
            let up = self.up_cell(column as usize);
            let left = self.left_cell(column as usize, &new);
            let cell = unsafe {
                // skip double mut borrow check
                &mut *(new.get_unchecked_mut(column as usize) as *mut Cell)
            };
            if cell.is_white() {
                continue;
            }
            if up.is_color(left.get_color()) && !up.is_white() {
                // 左边和上面是同一个
                if up.is_id(left.get_id()) {
                    // println!("情况 0");
                    cell.set_id(up.get_id())
                }
                // 左边, 上面, 此格, 皆同色
                // - 合并左边和上面的区域 id
                else if cell.is_color(up.get_color()) {
                    // println!("情况 1");
                    cell.set_id(self.merge_group(left, up, column));
                    unsafe {
                        let slice_mut = new.get_unchecked_mut(0..column as usize);
                        self.replace_id(slice_mut, left.get_id(), up.get_id());
                    }
                }
                else {
                    // println!("情况 3.1");
                    cell.set_id(self.create_group(column));
                }
            }
            else {
                // 和左边或者上面的格子之一颜色相同
                // 赋予颜色相同的块的 id
                if cell.is_color(up.get_color()) {
                    // println!("情况 2.1");
                    cell.set_id(self.insert_group(up, column));
                }
                else if cell.is_color(left.get_color()) {
                    // println!("情况 2.2");
                    cell.set_id(self.insert_group(left, column));
                }
                // 和左边以及上面的格子颜色都不同
                // 这是一个新的块, 赋予新的 id
                else {
                    // println!("情况 3.2");
                    cell.set_id(self.create_group(column));
                }
            }
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
    /// 合并两个区域
    fn merge_group(&mut self, left: Cell, up: Cell, column: u32) -> u32 {
        let rm = left.get_id();
        let keep = up.get_id();

        let rest = self.groups.merge_group(keep, rm);
        rest.push((self.lines, column));

        keep
    }

    fn replace_id(&mut self, view: &mut [Cell], old: u32, new: u32) {
        // println!("移除 {}, 插入 {}", old, new);
        for cell in self.last.iter_mut() {
            cell.replace_id(old, new);
        }
        for cell in view.iter_mut() {
            cell.replace_id(old, new);
        }
    }
    fn insert_group(&mut self, old: Cell, column: u32) -> u32 {
        let id = old.get_id();
        self.groups.insert(id, (self.lines, column));
        id
    }
    fn create_group(&mut self, column: u32) -> u32 {
        self.groups.create((self.lines, column))
    }
}

impl SquareSite<Uniform<u8>> {
    /// Create a new square site with given width and color
    pub fn uniform(width: usize, color: u8) -> Self {
        Self::new(width, Uniform::<u8>::new(0, color))
    }
}
