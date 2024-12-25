use std::{fs::read_to_string, thread, time::Duration};

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    velocity: (i64, i64),
}

const MAX_GRID_HEIGHT: i64 = 103;
const MAX_GRID_WIDTH: i64 = 101;
const MIDDLE_COL: usize = 50;
const MIDDLE_ROW: usize = 51;

// Looked at the output, notices similarities from 36, 137, 238 and so on..
fn matches_condition(n: i32) -> bool {
    if n == 37 {
        return true;
    }
    if (n - 37) % 101 == 0 {
        return true;
    }
    false
}

impl Robot {
    fn new(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let (mut pos, mut velocity) = (
            parts.next().unwrap().trim_start_matches("p=").split(","),
            parts.next().unwrap().trim_start_matches("v=").split(","),
        );

        Robot {
            position: (
                pos.next().unwrap().parse().unwrap(),
                pos.next().unwrap().parse().unwrap(),
            ),
            velocity: (
                velocity.next().unwrap().parse().unwrap(),
                velocity.next().unwrap().parse().unwrap(),
            ),
        }
    }
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day14/input.txt").unwrap();

    let mut robots: Vec<Robot> = vec![];

    puzzle_input.trim().split("\n").for_each(|line| {
        robots.push(Robot::new(line));
    });

    for _second in 0..10000 {
        let mut robot_matrix = [[false; MAX_GRID_WIDTH as usize]; MAX_GRID_HEIGHT as usize];
        for i in 0..robots.len() {
            let robot = &mut robots[i];

            let new_col = (robot.position.0 as i64 + robot.velocity.0) % MAX_GRID_WIDTH;
            let new_row = (robot.position.1 as i64 + robot.velocity.1) % MAX_GRID_HEIGHT;

            robot.position.0 = if new_col >= 0 {
                new_col
            } else {
                MAX_GRID_WIDTH + new_col
            } as usize;

            robot.position.1 = if new_row >= 0 {
                new_row
            } else {
                MAX_GRID_HEIGHT + new_row
            } as usize;

            robot_matrix[robot.position.1][robot.position.0] = true;
        }

        if matches_condition(_second) {
            thread::sleep(Duration::from_millis(200)); // stare at the output until you see a christmas tree
            println!("\n\nAfter {} seconds", _second);
            for row in robot_matrix {
                for col in row {
                    match col {
                        true => print!("X"),
                        false => print!(" "),
                    }
                }
                print!("\n")
            }
        }
    }

    let mut q1_count = 0;
    let mut q2_count = 0;
    let mut q3_count = 0;
    let mut q4_count = 0;

    for i in 0..robots.len() {
        let robot = &robots[i];
        if robot.position.0 < MIDDLE_COL {
            if robot.position.1 < MIDDLE_ROW {
                q1_count += 1;
            } else if robot.position.1 > MIDDLE_ROW {
                q3_count += 1;
            }
        } else if robot.position.0 > MIDDLE_COL {
            if robot.position.1 < MIDDLE_ROW {
                q2_count += 1;
            } else if robot.position.1 > MIDDLE_ROW {
                q4_count += 1;
            }
        }
    }

    println!("{}", q1_count * q2_count * q3_count * q4_count);
}
