use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Guard {
    row: i32,
    col: i32,
    direction: Direction,
}

impl Guard {
    fn turn(&mut self) {
        match self.direction {
            Direction::UP => self.direction = Direction::RIGHT,
            Direction::RIGHT => self.direction = Direction::DOWN,
            Direction::DOWN => self.direction = Direction::LEFT,
            Direction::LEFT => self.direction = Direction::UP,
        };
    }

    fn get_next_pos(&self) -> (i32, i32) {
        match self.direction {
            Direction::UP => (self.row - 1, self.col),
            Direction::DOWN => (self.row + 1, self.col),
            Direction::LEFT => (self.row, self.col - 1),
            Direction::RIGHT => (self.row, self.col + 1),
        }
    }

    fn move_to_next(&mut self) {
        let (next_row, next_col) = self.get_next_pos();

        self.row = next_row;
        self.col = next_col;
    }
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day06/input.txt").expect("Failed to open file");

    let map_content: Vec<Vec<String>> = puzzle_input
        .trim()
        .split("\n")
        .map(|ln| ln.split("").map(|el| el.to_string()).collect())
        .collect();

    let rows = map_content.len() as i32;
    let cols = map_content[0].len() as i32;

    let mut guard: Guard = {
        let mut found: Option<Guard> = None;
        'outer: for row in 0..rows {
            for col in 0..cols {
                match map_content[row as usize][col as usize].as_str() {
                    "<" => {
                        found = Some(Guard {
                            row,
                            col,
                            direction: Direction::LEFT,
                        });
                        break 'outer;
                    }
                    ">" => {
                        found = Some(Guard {
                            row,
                            col,
                            direction: Direction::RIGHT,
                        });
                        break 'outer;
                    }
                    "^" => {
                        found = Some(Guard {
                            row,
                            col,
                            direction: Direction::UP,
                        });
                        break 'outer;
                    }
                    "v" => {
                        found = Some(Guard {
                            row,
                            col,
                            direction: Direction::DOWN,
                        });
                        break 'outer;
                    }
                    _ => (),
                }
            }
        }

        found.expect("Could not find guard")
    };

    let mut unique_visited_tiles: HashSet<(i32, i32)> = HashSet::new();

    loop {
        unique_visited_tiles.insert((guard.row, guard.col));

        let (next_row, next_col) = guard.get_next_pos();

        if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
            break;
        }

        let next_item = map_content[next_row as usize][next_col as usize].as_str();

        if next_item == "#" {
            guard.turn()
        }

        guard.move_to_next()
    }

    println!("Part 1: {}", unique_visited_tiles.iter().len());
}
