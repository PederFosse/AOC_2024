use std::fs::read_to_string;

struct Button {
    x: i64,
    y: i64,
    times_pressed: i64,
}

impl Button {
    fn press(&mut self, sum: (i64, i64)) -> (i64, i64) {
        self.times_pressed += 1;
        (sum.0 + self.x, sum.1 + self.y)
    }

    fn unpress(&mut self, sum: (i64, i64)) -> (i64, i64) {
        self.times_pressed -= 1;
        (sum.0 - self.x, sum.1 - self.y)
    }
}

fn parse_machine_line(value: &str, value_separator: &str) -> (i64, i64) {
    let parsed: Vec<i64> = value.split(": ").collect::<Vec<&str>>()[1]
        .split(", ")
        .map(|v| {
            v.split(value_separator).collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap()
        })
        .collect();

    (parsed[0], parsed[1])
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day13/input.txt").unwrap();

    let machine_input = puzzle_input.split("\n\n");

    let mut total_tokens = 0;

    machine_input.for_each(|machine| {
        let lines: Vec<&str> = machine.split("\n").collect();

        let button_a = parse_machine_line(lines[0], "+");
        let button_b = parse_machine_line(lines[1], "+");
        let prize = parse_machine_line(lines[2], "=");

        let mut button_a = Button {
            x: button_a.0,
            y: button_a.1,
            times_pressed: 0,
        };

        let mut button_b = Button {
            x: button_b.0,
            y: button_b.1,
            times_pressed: 0,
        };

        // (x, y)
        let mut sum = (0, 0);

        // Spam the cheapest button
        while sum.0 < prize.0 || sum.1 < prize.1 {
            sum = button_b.press(sum);
        }

        loop {
            if sum.0 == prize.0 && sum.1 == prize.1 {
                total_tokens += button_a.times_pressed * 3 + button_b.times_pressed;
                break;
            }

            if button_b.times_pressed < 0 {
                break;
            }

            while sum.0 > prize.0 || sum.1 > prize.1 {
                sum = button_b.unpress(sum);
            }

            while sum.0 < prize.0 || sum.1 < prize.1 {
                sum = button_a.press(sum);
            }
        }
    });

    // part 2 requires some algebra. Another time.
    println!("Part 1: {}", total_tokens);
}
