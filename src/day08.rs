use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn main() {
    let contents = read_to_string("src/data/day08/input.txt").unwrap();

    let mut antenna_map: HashMap<&str, Vec<(isize, isize)>> = HashMap::new();

    let antenna_matrix: Vec<Vec<&str>> = contents
        .trim()
        .split("\n")
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect())
        .collect();

    for (row, line) in antenna_matrix.iter().enumerate() {
        for (col, &el) in line.iter().enumerate() {
            if el != "." {
                antenna_map
                    .entry(el)
                    .or_default()
                    .push((row as isize, col as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    let row_count = antenna_matrix.len() as isize;
    let col_count = antenna_matrix[0].len() as isize;

    for (_, antennas) in antenna_map {
        for (i, &(row, col)) in antennas.iter().enumerate() {
            for i in (i + 1)..antennas.len() {
                let (row2, col2) = antennas[i];

                let antinode1 = (row - (row2 - row), col - (col2 - col));
                let antinode2 = (row2 + (row2 - row), col2 + (col2 - col));

                if antinode1.0 >= 0
                    && antinode1.0 < row_count
                    && antinode1.1 >= 0
                    && antinode1.1 < col_count
                {
                    antinodes.insert(antinode1);
                }

                if antinode2.0 >= 0
                    && antinode2.0 < row_count
                    && antinode2.1 >= 0
                    && antinode2.1 < col_count
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
}
