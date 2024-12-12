use std::{collections::HashSet, fs::read_to_string};

struct Matrix {
    matrix: Vec<Vec<String>>,
    max_row: isize,
    max_col: isize,
}

impl Matrix {
    fn get_element(&self, pos: (isize, isize)) -> Option<i64> {
        let row = pos.0;
        let col = pos.1;

        if row < 0 || col < 0 {
            return None;
        }

        let Some(found_row) = self.matrix.get(row as usize) else {
            return None;
        };

        let Some(el) = found_row.get(col as usize) else {
            return None;
        };

        Some(el.parse().unwrap())
    }

    fn get_total_trail_score(&self) -> (usize, usize) {
        let mut score_part_2 = 0;
        let mut score_part_1 = 0;
        for row in 0..self.max_row {
            for col in 0..self.max_col {
                let val = self.matrix[row as usize][col as usize]
                    .parse::<i64>()
                    .unwrap();
                if val != 0 {
                    continue;
                }

                let Some(trail_ends) = trail_score(self, (row as isize, col as isize), val) else {
                    continue;
                };
                score_part_2 += trail_ends.len();
                score_part_1 += trail_ends
                    .into_iter()
                    .collect::<HashSet<(isize, isize)>>()
                    .len();
            }
        }

        (score_part_1, score_part_2)
    }
}

fn trail_score(
    matrix: &Matrix,
    pos: (isize, isize),
    current_value: i64,
) -> Option<Vec<(isize, isize)>> {
    if current_value == 9 {
        return Some(vec![pos]);
    }

    let mut trail_ends = vec![];
    for node in [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ] {
        let Some(next_val) = matrix.get_element(node) else {
            continue;
        };

        if next_val - current_value != 1 {
            continue;
        };

        let Some(trail_score) = trail_score(matrix, node, next_val) else {
            continue;
        };

        trail_score.iter().for_each(|&el| {
            trail_ends.push(el);
        });
    }

    Some(trail_ends)
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day10/input.txt").unwrap();

    let matrix_data: Vec<Vec<String>> = puzzle_input
        .trim()
        .split("\n")
        .map(|el| {
            el.split("")
                .filter(|el| !el.is_empty())
                .map(|el| el.to_string())
                .collect()
        })
        .collect();

    let matrix = Matrix {
        max_row: matrix_data.len() as isize,
        max_col: matrix_data[0].len() as isize,
        matrix: matrix_data,
    };

    let (part1, part2) = matrix.get_total_trail_score();

    println!("Part 1: {}, Part 2: {}", part1, part2);
}
