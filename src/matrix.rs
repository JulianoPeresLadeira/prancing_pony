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
            panic!("Calling 'set' out of bounds of matrix: {:?} on matrix of size ({}, {}).", pos, self.size_x, self.size_y);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let matrix = Matrix::new(2, 2);
        assert_eq!(matrix.values, vec![None, None, None, None]);
    }

    #[test]
    fn test_is_in_bounds() {
        let matrix = Matrix::new(3, 3);
        assert!(matrix.is_in_bounds((0, 0)));
        assert!(matrix.is_in_bounds((2, 2)));
        assert!(!matrix.is_in_bounds((3, 3)));
        assert!(!matrix.is_in_bounds((-1, -1)));
    }

    #[test]
    fn test_get() {
        let mut matrix = Matrix::new(2, 2);
        matrix.set((0, 0), 1);
        assert_eq!(matrix.get((0, 0)), Some(1));
        assert_eq!(matrix.get((1, 1)), None);
    }

    #[test]
    #[should_panic(expected = "Calling 'set' out of bounds of matrix: (2, 2) on matrix of size (2, 2).")]
    fn test_set_out_of_bounds() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut matrix = Matrix::new(2, 2);
        matrix.set((2, 2), 1);
    }

    #[test]
    #[should_panic(expected = "Calling 'reset' out of bounds of matrix")]
    fn test_reset_out_of_bounds() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut matrix = Matrix::new(2, 2);
        matrix.reset((2, 2));
    }

    #[test]
    fn test_reset() {
        let mut matrix = Matrix::new(2, 2);
        matrix.set((0, 0), 1);
        matrix.reset((0, 0));
        assert_eq!(matrix.get((0, 0)), None);
    }
}
