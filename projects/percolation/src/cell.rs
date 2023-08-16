use std::fmt::{Debug, Formatter};

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Cell {
    repr: [u8; 4],
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("color", &self.get_color())
            .field("id", &self.get_id())
            .finish()
    }
}

impl Cell {
    pub fn new(color: u8) -> Self {
        Self {
            repr: [color, 0, 0, 0],
        }
    }
    pub fn get_color(&self) -> u8 {
        self.repr[0]
    }
    pub fn set_color(&mut self, color: u8) {
        self.repr[0] = color;
    }
    pub fn get_id(&self) -> u32 {
        u32::from_be_bytes([self.repr[1], self.repr[2], self.repr[3], 0])
    }
    pub fn set_id(&mut self, id: u32) {
        let [a, b, c, _] = id.to_be_bytes();
        self.repr[1] = a;
        self.repr[2] = b;
        self.repr[3] = c;
    }
}