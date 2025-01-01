use core::panic;
use std::fs::read_to_string;

fn parse(input: String) -> (i64, i64, i64, Vec<usize>) {
    let mut parts = input.split("\n\n");

    let register: Vec<i64> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            split.next();
            split.next().unwrap().parse::<i64>().unwrap()
        })
        .collect();

    let mut program = parts.next().unwrap().split(": ");
    program.next();

    (
        register[0],
        register[1],
        register[2],
        program
            .next()
            .unwrap()
            .trim()
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect(),
    )
}

struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,

    operations: [fn(computer: &mut Self, operand: usize) -> Option<String>; 8],
}

fn adv(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    let numerator = computer.register_a;
    let denominator = 2i64.pow(computer.get_combo_operand(literal_operand).unwrap() as u32);
    computer.register_a = numerator / denominator;
    computer.instruction_pointer += 2;
    None
}

fn bxl(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    computer.register_b = computer.register_b ^ literal_operand as i64;
    computer.instruction_pointer += 2;
    None
}

fn bst(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    computer.register_b = computer.get_combo_operand(literal_operand).unwrap() % 8;
    computer.instruction_pointer += 2;
    None
}

fn jnz(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    if computer.register_a != 0 && computer.instruction_pointer != literal_operand as usize {
        computer.instruction_pointer = literal_operand as usize;
        return None;
    }

    computer.instruction_pointer += 2;
    None
}

fn bxc(computer: &mut Computer, _literal_operand: usize) -> Option<String> {
    computer.register_b = computer.register_b ^ computer.register_c;
    computer.instruction_pointer += 2;
    None
}

fn out(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    computer.instruction_pointer += 2;
    let result = computer.get_combo_operand(literal_operand).unwrap() % 8;
    Some(result.to_string())
}

fn bdv(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    let numerator = computer.register_a;
    let denominator = 2i64.pow(computer.get_combo_operand(literal_operand).unwrap() as u32);
    computer.register_b = numerator / denominator;
    computer.instruction_pointer += 2;
    None
}

fn cdv(computer: &mut Computer, literal_operand: usize) -> Option<String> {
    let numerator = computer.register_a;
    let denominator = 2i64.pow(computer.get_combo_operand(literal_operand).unwrap() as u32);
    computer.register_c = numerator / denominator;
    computer.instruction_pointer += 2;
    None
}

impl Computer {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Computer {
            register_a: a,
            register_b: b,
            register_c: c,
            operations: [adv, bxl, bst, jnz, bxc, out, bdv, cdv],
            instruction_pointer: 0,
        }
    }

    fn get_combo_operand(&self, literal_operand: usize) -> Option<i64> {
        match literal_operand {
            val if val <= 3 => Some(val as i64),
            4 => Some(self.register_a),
            5 => Some(self.register_b),
            6 => Some(self.register_c),
            7 => None,
            _ => panic!("Unexpected operand"),
        }
    }

    fn run_program(&mut self, program: Vec<usize>) -> String {
        let mut out = vec![];
        while self.instruction_pointer < program.len() - 1 {
            let operation = program[self.instruction_pointer];
            let operand = program[self.instruction_pointer + 1];

            match self.operations[operation](self, operand) {
                Some(output) => {
                    out.push(output);
                }
                _ => (),
            }
        }

        out.join(",")
    }
}

pub fn main() {
    println!("Advent of code day 17!");
    let puzzle_input = read_to_string("src/data/day17/input.txt").unwrap();

    let (a, b, c, program) = parse(puzzle_input);

    let mut computer = Computer::new(a, b, c);

    let out = computer.run_program(program);

    println!("Part 1: {}", out);
}
