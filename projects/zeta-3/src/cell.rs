use std::fmt::{Debug, Formatter};

/// The cell with color and ID, 0 means that there is no color, and at this time, the ID must be 0
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Cell {
    repr: [u8; 4],
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell").field("color", &self.get_color()).field("id", &self.get_id()).finish()
    }
}

impl Cell {
    /// Create a new cell with given color
    pub fn new(color: u8) -> Self {
        Self { repr: [color, 0, 0, 0] }
    }
    /// Create a new cell with given color and id
    pub fn get_color(&self) -> u8 {
        self.repr[0]
    }
    /// Create a new cell with given color and id
    pub fn is_color(&self, id: u8) -> bool {
        self.get_color() == id
    }
    /// Create a new cell with given color and id
    pub fn is_white(&self) -> bool {
        self.get_color() == 0
    }
    /// Create a new cell with given color and id
    pub fn set_color(&mut self, color: u8) {
        self.repr[0] = color;
    }
    /// Create a new cell with given color and id
    pub fn is_id(&self, id: u32) -> bool {
        self.get_id() == id
    }
    /// Get the id of this cell
    pub fn get_id(&self) -> u32 {
        u32::from_be_bytes([0, self.repr[1], self.repr[2], self.repr[3]])
    }
    /// Create a new cell with given color and id
    pub fn set_id(&mut self, id: u32) {
        let [_, a, b, c] = id.to_be_bytes();
        self.repr[1] = a;
        self.repr[2] = b;
        self.repr[3] = c;
    }
    /// Replace id `rm` with `rp`
    pub fn replace_id(&mut self, rm: u32, rp: u32) {
        if self.is_id(rm) {
            self.set_id(rp);
        }
    }
}
