use std::fs::read_to_string;

const MAX_STEP_SIZE: u32 = 3;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
}

fn is_report_safe(report: Vec<u32>, can_skip_one_step: bool) -> bool {
    let direction: Direction = {
        if report[0] < report[1] {
            Direction::UP
        } else {
            Direction::DOWN
        }
    };

    let mut prev_number: &u32 = &report[0];

    // Not guilty until proven otherwise
    let mut report_is_safe = true;

    let mut one_step_is_skipped = false;

    for (i, el) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let is_valid_step = match direction {
            Direction::UP => el > prev_number && el - prev_number <= MAX_STEP_SIZE,
            Direction::DOWN => el < prev_number && prev_number - el <= MAX_STEP_SIZE,
        };

        if !is_valid_step {
            if can_skip_one_step && one_step_is_skipped == false {
                one_step_is_skipped = true;
                continue;
            }

            report_is_safe = false;
            break;
        }

        prev_number = el;
    }
    println!(
        "Checked report: {:?}, direction: {:?}, safe: {}",
        report, direction, report_is_safe
    );
    report_is_safe
}

pub fn main() {
    let contents = read_to_string("src/data/day02/dummy.txt").expect("Unreadable file");
    let lines = contents.split("\n");

    let mut valid_report_count: u32 = 0;

    for report in lines {
        if is_report_safe(
            report
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
            false,
        ) {
            valid_report_count += 1;
        }
    }

    println!("Valid reports: {}", valid_report_count);
}
