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
    let mut antinodes_part_2: HashSet<(isize, isize)> = HashSet::new();

    let row_count = antenna_matrix.len() as isize;
    let col_count = antenna_matrix[0].len() as isize;

    for (_, antennas) in antenna_map {
        for (i, &(row, col)) in antennas.iter().enumerate() {
            for i in (i + 1)..antennas.len() {
                let (row2, col2) = antennas[i];

                let steps_x = row2 - row;
                let steps_y = col2 - col;

                // Step one way
                'one_way: {
                    let mut node: (isize, isize) = (row, col);
                    let mut part_1_added = false;
                    loop {
                        antinodes_part_2.insert(node);

                        node = (node.0 - steps_x, node.1 - steps_y);
                        if node.0 < 0 || node.0 >= row_count || node.1 < 0 || node.1 >= col_count {
                            break 'one_way;
                        }

                        if part_1_added == false {
                            antinodes.insert(node);
                            part_1_added = true;
                        }
                    }
                }

                // Step the other way
                'other_way: {
                    let mut node: (isize, isize) = (row2, col2); // ! row2 and col2
                    let mut part_1_added = false;
                    loop {
                        antinodes_part_2.insert(node);

                        node = (node.0 + steps_x, node.1 + steps_y);
                        if node.0 < 0 || node.0 >= row_count || node.1 < 0 || node.1 >= col_count {
                            break 'other_way;
                        }

                        if part_1_added == false {
                            antinodes.insert(node);
                            part_1_added = true;
                        }
                    }
                }
            }
        }
    }

    println!(
        "Part 1: {}, Part 2: {}",
        antinodes.len(),
        antinodes_part_2.len()
    );
}
