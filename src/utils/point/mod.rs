use direction::Direction;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }

        Some(Self {
            x: self.x,
            y: self.y - 1,
        })
    }

    pub fn down(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y + 1,
        })
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Self {
            x: self.x - 1,
            y: self.y,
        })
    }

    pub fn right(&self) -> Option<Self> {
        Some(Self {
            x: self.x + 1,
            y: self.y,
        })
    }

    pub fn get_point_in_direction(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::N => self.up(),
            Direction::S => self.down(),
            Direction::E => self.right(),
            Direction::W => self.left(),
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub mod direction;
