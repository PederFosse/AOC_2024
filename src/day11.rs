use std::{collections::HashMap, fs, mem};

fn blink(mut stones: HashMap<u64, u64>, blinks: u8) -> u64 {
    let mut new_map: HashMap<u64, u64> = HashMap::new();

    for _ in 0..blinks {
        for (stone, count) in stones.drain() {
            match stone {
                0 => {
                    *new_map.entry(1).or_default() += count;
                }
                _ if stone.to_string().len() % 2 == 0 => {
                    let stone_as_string = stone.to_string();
                    let (first, second) = stone_as_string.split_at(stone.to_string().len() / 2);
                    let first_as_i32 = first.parse::<u64>().unwrap();
                    let second_as_i32 = second.parse::<u64>().unwrap();

                    *new_map.entry(first_as_i32).or_default() += count;
                    *new_map.entry(second_as_i32).or_default() += count;
                }
                _ => {
                    *new_map.entry(stone * 2024).or_default() += count;
                }
            }
        }

        mem::swap(&mut stones, &mut new_map);
    }

    stones.values().sum()
}

pub fn main() {
    let puzzle_input: Vec<String> = fs::read_to_string("src/data/day11/input.txt")
        .unwrap()
        .split_whitespace()
        .map(|v| v.to_string())
        .collect();

    let mut stone_map = HashMap::new();

    for stone in puzzle_input {
        stone_map.insert(stone.parse::<u64>().unwrap(), 1);
    }

    let part1 = blink(stone_map.clone(), 25);
    let part2 = blink(stone_map, 75);
    println!("Part 1: {:?}, Part 2: {:?}", part1, part2);
}
