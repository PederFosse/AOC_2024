use std::{collections::HashSet, fs::read_to_string};

fn part1(disk_map: &Vec<&str>) -> Vec<isize> {
    let mut individual_blocks: Vec<isize> = vec![];
    for index in 0..disk_map.len() {
        let item = disk_map[index].parse().unwrap();
        match index % 2 {
            0 => {
                let id = (index / 2) as isize;
                for _ in 0..item {
                    individual_blocks.push(id);
                }
            }
            _ => {
                for _ in 0..item {
                    individual_blocks.push(-1);
                }
            }
        }
    }

    let mut back_index = individual_blocks.len();
    let mut compacted_result: Vec<isize> = vec![];
    for index in 0..individual_blocks.len() {
        if index >= back_index {
            break;
        }

        let item = individual_blocks[index];

        if item >= 0 {
            compacted_result.push(item);
            continue;
        }

        loop {
            back_index -= 1;
            if back_index < index {
                break;
            }

            let back_item = individual_blocks[back_index];

            if back_item >= 0 {
                compacted_result.push(back_item);
                break;
            }
        }
    }

    compacted_result
}

fn checksum(disk: Vec<isize>) -> i64 {
    let mut checksum = 0;

    for (index, val) in disk.iter().enumerate() {
        if val < &0 {
            continue;
        }
        checksum += (index as i64) * (*val as i64);
    }

    checksum
}

fn part2(disk: &Vec<&str>) -> Vec<isize> {
    // id = -1 means empty space
    let mut blocks: Vec<(isize, usize)> = vec![]; // (id, size)

    for index in 0..disk.len() {
        let size = disk[index].parse().unwrap();
        match index % 2 {
            0 => blocks.push(((index / 2) as isize, size)),
            _ => blocks.push((-1, size)),
        }
    }

    let mut compacted: Vec<isize> = vec![];
    let mut added_ids = HashSet::new();

    for i in 0..blocks.len() {
        let (id, size) = blocks[i];
        if id >= 0 && !added_ids.contains(&id) {
            added_ids.insert(id);
            for _ in 0..size {
                compacted.push(id)
            }
        } else {
            let mut remaining_size = size;
            let mut search_index = blocks.len() - 1;

            while search_index > 0 && remaining_size > 0 {
                let b = blocks[search_index];

                if b.0 >= 0 && b.1 <= remaining_size && !added_ids.contains(&b.0) {
                    for _ in 0..b.1 {
                        remaining_size -= 1;
                        compacted.push(b.0);
                        added_ids.insert(b.0);
                    }
                }

                search_index -= 1
            }

            for _ in 0..remaining_size {
                compacted.push(-1)
            }
        }
    }

    compacted
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day09/input.txt").unwrap();

    let disk_map: Vec<&str> = puzzle_input
        .trim()
        .split("")
        .filter(|v| !v.is_empty())
        .collect();

    let part1 = part1(&disk_map);
    let part2 = part2(&disk_map);

    println!("\nPart 1: {}, Part 2: {}", checksum(part1), checksum(part2));
}
