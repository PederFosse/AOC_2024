use std::fs::read_to_string;

const MAX_STEP_SIZE: u32 = 3;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
}

fn fuck_it_brute_force_part_2(report: &Vec<u32>) -> bool {
    let mut found_safe_report = false;
    for i in 0..report.len() {
        let mut with_one_item_removed = report.clone();
        with_one_item_removed.remove(i);

        if is_report_safe(&with_one_item_removed, Direction::UP)
            || is_report_safe(&with_one_item_removed, Direction::DOWN)
        {
            found_safe_report = true;
            break;
        }
    }

    found_safe_report
}

fn is_report_safe(report: &Vec<u32>, direction: Direction) -> bool {
    let mut prev_number: &u32 = &report[0];

    // Not guilty until proven otherwise
    let mut report_is_safe = true;

    for (i, el) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let is_valid_step = match direction {
            Direction::UP => el > prev_number && el - prev_number <= MAX_STEP_SIZE,
            Direction::DOWN => el < prev_number && prev_number - el <= MAX_STEP_SIZE,
        };

        if !is_valid_step {
            report_is_safe = false;
            break;
        }

        prev_number = el;
    }
    report_is_safe
}

pub fn main() {
    let contents = read_to_string("src/data/day02/input.txt").expect("Unreadable file");
    let lines = contents.split("\n");

    let mut valid_report_count: u32 = 0;
    let mut valid_report_count_part2: u32 = 0;

    for report in lines {
        let formatted_report: Vec<u32> = report
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if is_report_safe(&formatted_report, Direction::UP)
            || is_report_safe(&formatted_report, Direction::DOWN)
        {
            valid_report_count += 1;
        }

        if fuck_it_brute_force_part_2(&formatted_report) {
            valid_report_count_part2 += 1
        }
    }

    println!(
        "Part 1: {}, Part 2: {}",
        valid_report_count, valid_report_count_part2
    );
}
