use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 161;
    let expect_result_part2 = 48;

    let filename_example = "ex_input";
    let filename = "input";

    let contents_example =
        fs::read_to_string(filename_example).expect("Erro reading the EXAMPLE file");
    let contents = fs::read_to_string(filename).expect("ERROR reading the INPUT file");

    let mut succes;
    println!("Part 1 - Example");
    succes = execute_part(part1, &contents_example, expect_result_part1);
    println!("Part 1");
    succes = succes && execute_part(part1, &contents, 0);
    println!("Part 2 - Example");
    succes = succes && execute_part(part2, &contents_example, expect_result_part2);
    println!("Part 2");
    let _ = succes && execute_part(part2, &contents, 0);
}

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32) -> bool {
    let start = Instant::now();
    let result = part_fn(&input);

    println!("\tFinished after {:?}", start.elapsed());
    println!("\tSolution found: {}", result);

    if example_result == 0 {
        println!("\tSkipping check");
        true
    } else if result != example_result {
        println!(
            "\tINCORRECT: Expected: [{}] but got [{}]",
            example_result, result
        );
        false
    } else {
        println!("\tCORRECT");
        true
    }
}

use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|mat| {
            mat.iter()
                .skip(1)
                .filter_map(|g| g)
                .map(|g| g.as_str().parse::<u32>().expect("not int"))
                .fold(1, |acc, d| acc * d)
        })
        .sum()
}
#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}
#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    start_pos: usize,
}

fn part2(input: &str) -> u32 {
    let mut instructions: Vec<Instruction> = vec![];
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don\'t\(\)").unwrap();

    re_mul.captures_iter(input).for_each(|mat| {
        instructions.push(Instruction {
            operation: Operation::Mul {
                0: mat.get(1).expect("").as_str().parse().unwrap(),
                1: mat.get(2).expect("").as_str().parse().unwrap(),
            },
            start_pos: mat.get(0).expect("").start(),
        });
    });

    re_do.find_iter(input).for_each(|mat| {
        instructions.push(Instruction {
            operation: Operation::Do,
            start_pos: mat.start(),
        });
    });

    re_dont.find_iter(input).for_each(|mat| {
        instructions.push(Instruction {
            operation: Operation::Dont,
            start_pos: mat.start(),
        });
    });

    instructions.sort_by_key(|item| item.start_pos);
    
    instructions
        .iter()
        .fold((0, true), |(acc, todo), op| match op.operation {
            Operation::Mul(x, y) => {
                if todo {
                    (acc + x * y, todo)
                } else {
                    (acc, todo)
                }
            }
            Operation::Do => (acc, true),
            Operation::Dont => (acc, false),
        })
        .0 as u32
}
