use std::fs::read_to_string;

#[derive(Debug)]
struct Robot {
    row: usize,
    col: usize,
}

pub fn main() {
    println!("\nAdvent of code day 15!");
    let puzzle_input = read_to_string("src/data/day15/input.txt").unwrap();

    let mut parts: std::str::Split<'_, &str> = puzzle_input.split("\n\n");

    let mut warehouse: Vec<Vec<&str>> = parts
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| {
            line.split("")
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();

    let mut robot: Robot = {
        // Find the robot
        let mut found: Option<Robot> = None;
        'search: for row in 0..warehouse.len() {
            for col in 0..warehouse[0].len() {
                if warehouse[row][col].eq("@") {
                    found = Some(Robot { row, col });
                    break 'search;
                }
            }
        }

        found.unwrap()
    };

    let instructions = parts
        .next()
        .unwrap()
        .split("")
        .filter(|&v| !v.is_empty() && v != "\n");

    for direction in instructions {
        match direction {
            "<" => {
                let mut current_col = robot.col;

                loop {
                    current_col -= 1;
                    let current_val = warehouse[robot.row][current_col];

                    if current_val == "#" {
                        break;
                    }

                    if current_val == "." {
                        warehouse[robot.row][robot.col] = ".";
                        robot.col = robot.col - 1;
                        warehouse[robot.row][robot.col] = "@";
                        if current_col != robot.col {
                            warehouse[robot.row][current_col] = "O";
                        }
                        break;
                    }
                }
            }
            ">" => {
                let mut current_col = robot.col;

                loop {
                    current_col += 1;
                    let current_val = warehouse[robot.row][current_col];

                    if current_val == "#" {
                        break;
                    }

                    if current_val == "." {
                        warehouse[robot.row][robot.col] = ".";
                        robot.col = robot.col + 1;
                        warehouse[robot.row][robot.col] = "@";
                        if current_col != robot.col {
                            warehouse[robot.row][current_col] = "O";
                        }
                        break;
                    }
                }
            }
            "^" => {
                let mut current_row = robot.row;
                loop {
                    current_row -= 1;
                    let current_val = warehouse[current_row][robot.col];

                    if current_val == "#" {
                        break;
                    }

                    if current_val == "." {
                        warehouse[robot.row][robot.col] = ".";
                        robot.row = robot.row - 1;
                        warehouse[robot.row][robot.col] = "@";
                        if current_row != robot.row {
                            warehouse[current_row][robot.col] = "O";
                        }
                        break;
                    }
                }
            }
            "v" => {
                let mut current_row = robot.row;
                loop {
                    current_row += 1;
                    let current_val = warehouse[current_row][robot.col];

                    if current_val == "#" {
                        break;
                    }

                    if current_val == "." {
                        warehouse[robot.row][robot.col] = ".";
                        robot.row = robot.row + 1;
                        warehouse[robot.row][robot.col] = "@";
                        if current_row != robot.row {
                            warehouse[current_row][robot.col] = "O";
                        }
                        break;
                    }
                }
            }
            _ => (),
        }
    }

    let mut gps_coordinates_sum = 0;

    // Calculate coordinates
    for row in 0..warehouse.len() {
        for col in 0..warehouse[0].len() {
            if warehouse[row][col] == "O" {
                gps_coordinates_sum += 100 * row + col;
            }
        }
    }

    // Print warehouse
    /* for row in 0..warehouse.len() {
        for col in 0..warehouse[0].len() {
            print!("{}", warehouse[row][col]);
        }
        print!("\n");
    } */

    println!("Part 1: {}", gps_coordinates_sum);
}
