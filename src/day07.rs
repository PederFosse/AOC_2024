use std::fs::read_to_string;

struct Equation {
    target: u64,
    values: Vec<u64>,
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}
fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn solve(
    values: &Vec<u64>,
    index: usize,
    acc: u64,
    target: u64,
    operations: &Vec<fn(a: u64, b: u64) -> u64>,
) -> Option<u64> {
    if acc == target {
        return Some(acc);
    }

    // Exit early if target is passed, as we never subtract
    if acc > target {
        return None;
    }

    if index == values.len() {
        return None;
    }

    for op in operations {
        let new_acc = op(acc, values[index]);
        let result = solve(values, index + 1, new_acc, target, operations);
        if result.is_some() {
            return result;
        }
    }

    None
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day07/input.txt").expect("Failed to open file");
    let mut acc_part1 = 0;
    let mut acc_part2 = 0;
    puzzle_input.trim().split("\n").for_each(|line| {
        let mut split_line = line.split(": ");

        let equation = Equation {
            target: split_line.next().unwrap().parse().unwrap(),
            values: split_line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|el| el.parse().unwrap())
                .collect(),
        };

        let result_part1 = solve(
            &equation.values,
            1,
            equation.values[0],
            equation.target,
            &vec![add, multiply],
        );

        match result_part1 {
            Some(value) => acc_part1 += value,
            _ => (),
        }

        let result_part2 = solve(
            &equation.values,
            1,
            equation.values[0],
            equation.target,
            &vec![add, multiply, concat],
        );

        match result_part2 {
            Some(value) => acc_part2 += value,
            _ => (),
        }
    });

    println!("Part 1: {}, Part 2: {}", acc_part1, acc_part2)
}
