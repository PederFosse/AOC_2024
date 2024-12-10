use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let days = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
        day09::main,
        day10::main,
    ];

    let now = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let day = args[1]
        .parse::<usize>()
        .expect("Day must be a valid integer");

    days[day - 1]();
    println!("Time elapsed: {}", now.elapsed().as_micros())
}
