use std::{collections::HashSet, fmt, fs::read_to_string};

fn parse_puzzle_input(path: &str) -> Vec<Vec<String>> {
    read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|line| {
            line.split("")
                .filter(|el| !el.is_empty())
                .map(|el| el.to_string())
                .collect()
        })
        .collect()
}

#[derive(Hash, Eq, PartialEq)]
struct Point<'a> {
    row: usize,
    col: usize,
    garden: &'a Vec<Vec<String>>,
}

impl<'a> Point<'a> {
    fn new(row: usize, col: usize, garden: &'a Vec<Vec<String>>) -> Self {
        Point { row, col, garden }
    }

    fn value(&self) -> &String {
        self.garden.get(self.row).unwrap().get(self.col).unwrap()
    }
}

impl<'a> fmt::Debug for Point<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}): {}", self.row, self.col, self.value())
    }
}

fn plot_price(
    start_pos: (usize, usize),
    garden: &Vec<Vec<String>>,
    visited: &mut HashSet<(usize, usize)>,
    height: usize,
    width: usize,
) -> i32 {
    let mut queue: Vec<Point> = vec![];
    let mut fence_count = 0;
    let visited_before = visited.len();
    queue.push(Point::new(start_pos.0, start_pos.1, garden));

    while !queue.is_empty() {
        let item = queue.pop().unwrap();
        if visited.contains(&(item.row, item.col)) {
            continue;
        }

        visited.insert((item.row, item.col));

        let value = item.value();

        let mut adjacent_points = [const { None }; 4];

        if item.row > 0 {
            adjacent_points[0] = Some(Point::new(item.row - 1, item.col, garden));
        }

        if item.row < height - 1 {
            adjacent_points[1] = Some(Point::new(item.row + 1, item.col, garden));
        }

        if item.col > 0 {
            adjacent_points[2] = Some(Point::new(item.row, item.col - 1, garden));
        }

        if item.col < width - 1 {
            adjacent_points[3] = Some(Point::new(item.row, item.col + 1, garden));
        }

        let mut fences: i32 = 4;
        for adjacent_point in adjacent_points {
            let Some(point) = adjacent_point else {
                continue;
            };
            if !point.value().eq(value) {
                continue;
            }

            fences -= 1;

            if !visited.contains(&(point.row, point.col)) {
                queue.push(point);
            }
        }

        fence_count += fences;
    }

    let area = (visited.len() - visited_before) as i32;

    fence_count * area
}

pub fn main() {
    let garden_map = parse_puzzle_input("src/data/day12/input.txt");

    let height = garden_map.len();
    let width = garden_map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price = 0;
    for row in 0..height {
        for col in 0..width {
            if !visited.contains(&(row, col)) {
                let plot_result = plot_price((row, col), &garden_map, &mut visited, height, width);

                total_price += plot_result;
            }
        }
    }

    println!("Total price: {}", total_price);
}
