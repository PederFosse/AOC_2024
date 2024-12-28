#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone, Copy)]
pub enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Direction {
    pub fn rotate_90(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    pub fn rotate_counter_90(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
}
