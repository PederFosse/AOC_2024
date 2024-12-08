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

fn solve(
    values: &Vec<u64>,
    index: usize,
    acc: u64,
    target: u64,
    operation: fn(a: u64, b: u64) -> u64,
) -> Option<u64> {
    /* println!(
        "Solving for values: {:?} with index {}, acc: {}, target: {}, operation: {}",
        values, index, acc, target, op_name
    ); */
    if acc == target {
        return Some(acc);
    }

    if index == values.len() {
        return None;
    }

    let new_acc = operation(acc, values[index]);

    if new_acc > target {
        return None;
    }

    let add_result = solve(values, index + 1, new_acc, target, add);
    if add_result.is_some() {
        return add_result;
    }

    let multiply_result = solve(values, index + 1, new_acc, target, multiply);
    if multiply_result.is_some() {
        return multiply_result;
    }

    None
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day07/input.txt").expect("Failed to open file");
    let mut total_acc = 0;
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

        let result1 = solve(
            &equation.values,
            1,
            equation.values[0],
            equation.target,
            multiply,
        );
        let result2 = solve(
            &equation.values,
            1,
            equation.values[0],
            equation.target,
            add,
        );

        match result1 {
            Some(value) => total_acc += value,
            _ => match result2 {
                Some(value) => total_acc += value,
                _ => (),
            },
        }
    });

    println!("Part 1: {}", total_acc)
}
