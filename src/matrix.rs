pub struct Matrix {
    size_x: u8,
    size_y: u8,
    values: Vec<Option<u8>>,
}

impl Matrix {
    pub fn new(size_x: u8, size_y: u8) -> Self {
        let size = size_y as usize * size_x as usize;
        let values: Vec<Option<u8>> = vec![None; size];
        Self { size_x, size_y, values }
    }

    pub fn is_in_bounds(&self, pos: (i16,i16)) -> bool {
        pos.0 >= 0 && pos.0 < self.size_x as i16 && pos.1 >= 0 && pos.1 < self.size_y as i16
    }

    pub fn get(&self, pos: (i16, i16)) -> Option<u8> {
        self.values[self.to_index(pos)]
    }

    pub fn set(&mut self, pos: (i16, i16), value: u8) {
        if !self.is_in_bounds(pos) {
            panic!("Calling 'set' out of bounds of matrix: ({:?}) on matrix of size ({}, {}).", pos, self.size_x, self.size_y);
        }
        
        let index = self.to_index(pos);
        self.values[index] = Some(value);
    }

    pub fn reset(&mut self, pos: (i16, i16)) {
        if !self.is_in_bounds(pos) {
            panic!("Calling 'reset' out of bounds of matrix: ({:?}) on matrix of size ({}, {}).", pos, self.size_x, self.size_y);
        }
        
        let index = self.to_index(pos);
        self.values[index] = None;
    }

    fn to_index(&self, pos: (i16, i16)) -> usize {
        pos.1 as usize * self.size_x as usize + pos.0 as usize
    }
}
