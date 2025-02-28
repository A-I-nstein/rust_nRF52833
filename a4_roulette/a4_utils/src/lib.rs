#![no_std]

pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub struct DisplayData {
    pub curr_pos: (isize, isize),
    pub direction: Direction,
}

impl DisplayData {
    pub const GRID_SIZE: isize = 5;

    fn is_valid_pos(pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < Self::GRID_SIZE && pos.1 >= 0 && pos.1 < Self::GRID_SIZE
    }

    pub fn get_next_pos(&mut self) {
        let next_pos = match self.direction {
            Direction::Right => (self.curr_pos.0, self.curr_pos.1 + 1),
            Direction::Down => (self.curr_pos.0 + 1, self.curr_pos.1),
            Direction::Left => (self.curr_pos.0, self.curr_pos.1 - 1),
            Direction::Up => (self.curr_pos.0 - 1, self.curr_pos.1),
        };

        if Self::is_valid_pos(&next_pos) {
            self.curr_pos = next_pos;
        } else {
            self.direction = match self.direction {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            };
            self.get_next_pos();
        }
    }
}
