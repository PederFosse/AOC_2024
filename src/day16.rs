use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

use crate::utils::{
    grid::Grid,
    point::{direction::Direction, Point},
};

#[derive(PartialEq, Eq, Debug)]
struct State {
    cost: usize,
    point: Point,
    direction: Direction,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

// Order is reversed for min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn parse(puzzle_input: String) -> (Grid<bool>, Point, Point) {
    let mut vec = vec![];
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let mut y: usize = 0;
    for line in puzzle_input.trim().lines() {
        for (x, val) in line.bytes().enumerate() {
            match val {
                b'#' => vec.push(false),
                b'.' => vec.push(true),
                b'S' => {
                    vec.push(true);
                    start = Some(Point::new(x, y));
                }
                b'E' => {
                    vec.push(true);
                    end = Some(Point::new(x, y));
                }
                _ => panic!("Found unexpected value when parsing grid"),
            }
        }
        y += 1
    }

    (Grid::from(vec, y), start.unwrap(), end.unwrap())
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day16/input.txt").unwrap();

    let (grid, start, end) = parse(puzzle_input);
    let mut costs: HashMap<(Point, Direction), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        direction: Direction::E,
        point: start,
    });

    while !heap.is_empty() {
        let current = heap.pop().unwrap();

        // Skip if 'cheaper' way already exists
        let existing_cost = costs
            .get(&(current.point, current.direction))
            .unwrap_or(&usize::MAX);
        if existing_cost < &current.cost {
            continue;
        }

        costs.insert((current.point, current.direction), current.cost);

        let options = [
            (
                grid.go_if_true(&current.point, current.direction),
                current.cost + 1,
                current.direction,
            ),
            (
                grid.go_if_true(&current.point, current.direction.rotate_90()),
                current.cost + 1001,
                current.direction.rotate_90(),
            ),
            (
                grid.go_if_true(&current.point, current.direction.rotate_counter_90()),
                current.cost + 1001,
                current.direction.rotate_counter_90(),
            ),
        ];

        for option in options {
            let (Some(point), cost, direction) = option else {
                continue;
            };

            heap.push(State {
                cost,
                point,
                direction,
            })
        }
    }

    let mut part1 = None;
    for cost in [
        costs.get(&(end, Direction::N)),
        costs.get(&(end, Direction::S)),
        costs.get(&(end, Direction::E)),
        costs.get(&(end, Direction::W)),
    ] {
        let Some(score) = cost else {
            continue;
        };

        if part1.is_none() || score < part1.unwrap() {
            part1 = Some(score);
        }
    }

    println!("Part 1: {}", part1.unwrap());
}
