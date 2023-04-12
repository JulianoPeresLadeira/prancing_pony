use crate::matrix::Matrix;
use crate::position::Position;
use crate::moves::MOVES;
pub struct TourInput {
    pub size_x: u8,
    pub size_y: u8,
    pub starting_position: (i16, i16)
}

pub enum TourResult {
    NoSolution,
    Solution(Tour)
}

pub struct Tour {
    pub position_history: Vec<Position>,
    pub times_backtracked: u32
}

pub struct TourGuide {
    visitation_history: Matrix,
    tour_size: usize,
    position_history: Vec<Position>,
    current_position: Position,
    times_backtracked: u32
}

impl TourGuide {

    fn new(inp: TourInput) -> TourGuide {
        let starting_position = Position(inp.starting_position.0, inp.starting_position.1);
        let tour_size = inp.size_x as usize * inp.size_y as usize;
        let visitation_history = Matrix::new(inp.size_x, inp.size_y);
        let mut position_history: Vec<Position> = Vec::with_capacity(tour_size);

        position_history.push(starting_position);

        TourGuide {
            visitation_history, 
            tour_size, 
            position_history, 
            current_position: starting_position,
            times_backtracked: 0
        }
    }

    fn calc_tour(&mut self) {
        while !self.tour_is_finished() {
            let next_move = self.next_lowest_indexed_valid_move();

            match next_move {
                None => self.backtrack(),
                Some(move_index) => self.move_knight(move_index)
            };
        }
    }

    fn next_lowest_indexed_valid_move(&self) -> Option<u8> {
        let mut i: usize = self
            .visitation_history
            .get(self.current_position.to_tuple())
            .map_or(0, |x| x + 1) 
            as usize;

        while i < MOVES.len() {
            let movement = &MOVES[i];
            let new_pos = self.current_position.calc_move(movement);

            if self.visitation_history.is_in_bounds(new_pos.to_tuple()) && self.visitation_history.get(new_pos.to_tuple()).is_none() {
                return Some(i as u8);
            }

            i += 1;
        }

        None
    }

    fn move_knight(&mut self, movement_index: u8) {
        let movement = &MOVES[movement_index as usize];
        let new_pos = self.current_position.calc_move(movement);
        self.visitation_history.set(self.current_position.to_tuple(), movement_index);
        self.position_history.push(new_pos);
        self.current_position = new_pos;
    }

    fn backtrack(&mut self) {
        self.visitation_history.reset(self.current_position.to_tuple());
        self.position_history.pop();
        self.current_position = *self.position_history.last().unwrap();
        self.times_backtracked += 1;
    }

    fn tour_is_finished(&self) -> bool {
        self.position_history.len() == self.tour_size
    }

    fn run_tour(inp: TourInput) -> TourResult {
        let mut guide = TourGuide::new(inp);
        guide.calc_tour();

        TourResult::Solution(
            Tour {
                position_history: guide.position_history, 
                times_backtracked: guide.times_backtracked
            }
        )
    }
}

fn has_solution(size_x: u8, size_y: u8) -> bool {

    if size_x > size_y {
        return has_solution(size_y, size_x);
    }

    let both_are_odd = size_x % 2 == 1 && size_y % 2 == 1;
    let forbidden_x_values = size_x == 1 || size_x == 2 || size_x == 4;
    let forbidden_y_value_for_x = (size_x == 3) && (size_y == 4 || size_y == 6 || size_y == 8);

    !(both_are_odd || forbidden_x_values || forbidden_y_value_for_x)
}

pub fn find_solution(tour: TourInput) -> TourResult {
    
    if !has_solution(tour.size_x, tour.size_y) {
        return TourResult::NoSolution;
    }

    TourGuide::run_tour(tour)
}
