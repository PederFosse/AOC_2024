use std::fs::read_to_string;

struct WordSearch {
    matrix: Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
}

impl WordSearch {
    pub fn new(contents: String) -> Self {
        let matrix: Vec<Vec<char>> = contents
            .trim()
            .split("\n")
            .map(|f| f.chars().collect::<Vec<char>>())
            .collect();

        let row_count = matrix.len();
        let col_count = matrix[0].len();

        Self {
            matrix,
            row_count,
            col_count,
        }
    }

    pub fn get_item(&self, row: usize, col: usize) -> Option<char> {
        if self.row_count < row || self.col_count < col {
            return None;
        }
        let item = self.matrix[row][col];
        Some(item)
    }
}

fn part_one(word_search: &WordSearch) -> u32 {
    let mut word_count = 0;
    for row in 0..word_search.row_count {
        for col in 0..word_search.col_count {
            let Some(el) = word_search.get_item(row, col) else {
                panic!("Could not find element")
            };

            if el != 'X' {
                continue;
            }

            // To the left
            if col >= 3 {
                match (
                    word_search.get_item(row, col - 1),
                    word_search.get_item(row, col - 2),
                    word_search.get_item(row, col - 3),
                ) {
                    (Some('M'), Some('A'), Some('S')) => word_count += 1,
                    _ => (),
                }
            }

            // To the right
            if word_search.col_count - col > 3 {
                match (
                    word_search.get_item(row, col + 1),
                    word_search.get_item(row, col + 2),
                    word_search.get_item(row, col + 3),
                ) {
                    (Some('M'), Some('A'), Some('S')) => word_count += 1,
                    _ => (),
                }
            }

            // Up, up-right and up-left
            if row >= 3 {
                // Up
                match (
                    word_search.get_item(row - 1, col),
                    word_search.get_item(row - 2, col),
                    word_search.get_item(row - 3, col),
                ) {
                    (Some('M'), Some('A'), Some('S')) => word_count += 1,
                    _ => (),
                }

                // Up-right
                if word_search.col_count - col > 3 {
                    match (
                        word_search.get_item(row - 1, col + 1),
                        word_search.get_item(row - 2, col + 2),
                        word_search.get_item(row - 3, col + 3),
                    ) {
                        (Some('M'), Some('A'), Some('S')) => word_count += 1,
                        _ => (),
                    }
                }

                if col >= 3 {
                    // Up-left
                    match (
                        word_search.get_item(row - 1, col - 1),
                        word_search.get_item(row - 2, col - 2),
                        word_search.get_item(row - 3, col - 3),
                    ) {
                        (Some('M'), Some('A'), Some('S')) => word_count += 1,
                        _ => (),
                    }
                }
            }

            // Down, down-right and down-left
            if word_search.row_count - row > 3 {
                // Down
                match (
                    word_search.get_item(row + 1, col),
                    word_search.get_item(row + 2, col),
                    word_search.get_item(row + 3, col),
                ) {
                    (Some('M'), Some('A'), Some('S')) => word_count += 1,
                    _ => (),
                }

                // Up-right
                if word_search.col_count - col > 3 {
                    match (
                        word_search.get_item(row + 1, col + 1),
                        word_search.get_item(row + 2, col + 2),
                        word_search.get_item(row + 3, col + 3),
                    ) {
                        (Some('M'), Some('A'), Some('S')) => word_count += 1,
                        _ => (),
                    }
                }

                if col >= 3 {
                    // Up-left
                    match (
                        word_search.get_item(row + 1, col - 1),
                        word_search.get_item(row + 2, col - 2),
                        word_search.get_item(row + 3, col - 3),
                    ) {
                        (Some('M'), Some('A'), Some('S')) => word_count += 1,
                        _ => (),
                    }
                }
            }
        }
    }

    word_count
}

fn part_two(word_search: &WordSearch) -> u32 {
    let mut xmas_count = 0;

    for row in 0..word_search.row_count {
        for col in 0..word_search.col_count {
            let Some(el) = word_search.get_item(row, col) else {
                panic!("Could not find element")
            };

            if el != 'A' {
                continue;
            }

            if col < 1
                || row < 1
                || col >= word_search.col_count - 1
                || row >= word_search.row_count - 1
            {
                continue;
            }

            let diagonal_match_1 = match (
                word_search.get_item(row - 1, col - 1),
                word_search.get_item(row + 1, col + 1),
            ) {
                (Some('M'), Some('S')) => true,
                (Some('S'), Some('M')) => true,
                _ => false,
            };

            let diagonal_match_2 = match (
                word_search.get_item(row - 1, col + 1),
                word_search.get_item(row + 1, col - 1),
            ) {
                (Some('M'), Some('S')) => true,
                (Some('S'), Some('M')) => true,
                _ => false,
            };

            if diagonal_match_1 && diagonal_match_2 {
                println!("Found x-mas with A on position: ({},{})", row, col);
                xmas_count += 1
            }
        }
    }

    xmas_count
}
pub fn main() {
    println!("Advent of code, day 4");
    let contents = read_to_string("src/data/day04/input.txt").unwrap();

    let word_search = WordSearch::new(contents);

    println!(
        "Part 1: {}, Part 2: {}",
        part_one(&word_search),
        part_two(&word_search)
    )
}
