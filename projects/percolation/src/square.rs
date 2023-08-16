pub struct Cell {
    repr: [u8; 4],
}

impl Cell {
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
        self.repr[1..4].copy_from_slice(&id.to_be_bytes());
    }
}