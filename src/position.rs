use std::fmt::{Display, Formatter, Result};
use crate::moves::Move;

#[derive(Copy,Clone,Debug)]

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn calc_move(&self, dp: &Move) -> Position {
        let new_x = self.0 + dp.0 as i16;
        let new_y = self.1 + dp.1 as i16;

        Position(new_x, new_y)
    }

    pub fn to_tuple(&self) -> (i16, i16) {
        (self.0, self.1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}