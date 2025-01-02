use std::ops::Index;

use super::point::{direction::Direction, Point};

pub struct Grid<T> {
    height: usize,
    width: usize,
    vec: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from(vec: Vec<T>, height: usize) -> Self {
        Grid {
            height,
            width: vec.len() / height,
            vec,
        }
    }

    pub fn same_size_with<TNew>(&self, fill_value: TNew) -> Grid<TNew>
    where
        TNew: Clone,
    {
        Grid {
            vec: vec![fill_value; self.height * self.width],
            height: self.height,
            width: self.width,
        }
    }

    pub fn update(&mut self, pos: (usize, usize), value: T) {
        self.vec[self.width * pos.0 + pos.1] = value
    }

    fn is_within_grid(&self, point: Point) -> bool {
        point.x < self.width && point.y < self.height
    }
}

impl Grid<bool> {
    pub fn go_if_true(&self, point: &Point, direction: Direction) -> Option<Point> {
        let Some(next_point) = point.get_point_in_direction(direction) else {
            return None;
        };

        if !self.is_within_grid(next_point) {
            return None;
        }

        match self[&next_point] {
            true => Some(next_point),
            false => None,
        }
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[(index.x, index.y)]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.vec[self.width * y + x]
    }
}

impl std::fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for y in 0..self.height {
            str.push_str(&format!("{:3} ", y % 1000));
            for x in 0..self.width {
                match self[(x, y)] {
                    true => str.push_str("■ "),
                    false => str.push_str("· "),
                }
            }
            str.push('\n');
        }
        str.push_str("    ");
        for x in 0..self.width {
            str.push_str(&format!("{:} ", x % 10));
        }

        write!(f, "{}", str)
    }
}
