use regex::Regex;
use std::fs::read_to_string;

fn calculate(contents: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let result = re.find_iter(contents);

    let mut acc = 0;

    for el in result {
        let value = el.as_str();
        let (number1, number2) = {
            let mut split = value
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split(",");

            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        };

        acc += number1 * number2;
    }

    acc
}

fn calculate_part_2(contents: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();

    let result = re.find_iter(contents);

    let mut acc = 0;

    let mut should_do = true;

    for el in result {
        let value = el.as_str();

        match value {
            "don't()" => should_do = false,
            "do()" => should_do = true,
            _ => {
                if should_do == false {
                    continue;
                }
                let (number1, number2) = {
                    let mut split = value
                        .trim_start_matches("mul(")
                        .trim_end_matches(")")
                        .split(",");

                    (
                        split.next().unwrap().parse::<i32>().unwrap(),
                        split.next().unwrap().parse::<i32>().unwrap(),
                    )
                };

                acc += number1 * number2;
            }
        }
    }

    acc
}

pub fn main() {
    println!("Advent of code, day 3");

    let contents = read_to_string("src/data/day03/input.txt").unwrap();

    let part1 = calculate(&contents);
    let part2 = calculate_part_2(&contents);

    println!("Part 1: {part1}, part 2: {part2}");
}
