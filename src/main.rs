use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let days = [day01::main, day02::main, day03::main, day04::main];

    let now = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let day = args[1]
        .parse::<usize>()
        .expect("Day must be a valid integer");

    days[day - 1]();
    println!("Time elapsed: {}", now.elapsed().as_micros())
}
