use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

use crate::utils::{
    grid::Grid,
    point::{direction::Direction, Point},
};

/*
// For dummy:
const GRID_HEIGHT: usize = 7;
const BYTES: usize = 12;
*/

const GRID_HEIGHT: usize = 71;

// For part two I just did a manual binary search to see when the dijkstra result
// equals 'None' by updating this BYTES value until I reached 3001
const BYTES: usize = 1024;

#[derive(PartialEq, Eq)]
struct Node {
    cost: i64,
    point: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn parse(input: String) -> Grid<bool> {
    let vec = vec![true; GRID_HEIGHT * GRID_HEIGHT];

    let mut grid = Grid::from(vec, GRID_HEIGHT);

    for (index, line) in input.lines().enumerate() {
        if index == BYTES {
            break;
        }
        let mut parts = line.split(",");
        let (y, x) = (
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        );

        grid.update((x, y), false);
    }

    grid
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day18/input.txt").unwrap();
    let grid = parse(puzzle_input);

    let mut heap = BinaryHeap::new();
    heap.push(Node {
        point: Point::new(0, 0),
        cost: 0,
    });

    let mut costs: HashMap<Point, i64> = HashMap::new();

    while let Some(Node { point, cost }) = heap.pop() {
        // Skip if cheaper exists
        let existing_cost = costs.get(&point).unwrap_or(&i64::MAX);
        if existing_cost <= &cost {
            continue;
        }

        costs.insert(point, cost);

        let options = [
            grid.go_if_true(&point, Direction::N),
            grid.go_if_true(&point, Direction::S),
            grid.go_if_true(&point, Direction::E),
            grid.go_if_true(&point, Direction::W),
        ];

        for option in options {
            let Some(point) = option else {
                continue;
            };

            heap.push(Node {
                point,
                cost: cost + 1,
            });
        }
    }

    println!(
        "Part 1: {:?}",
        costs.get(&Point::new(GRID_HEIGHT - 1, GRID_HEIGHT - 1))
    )
}
