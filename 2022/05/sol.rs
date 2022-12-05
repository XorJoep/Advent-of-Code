use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 0;
    let expect_result_part2 = 0;

    let filename_example = "ex_input";
    let filename = "input";

    let contents_example = fs
        ::read_to_string(filename_example)
        .expect("Erro reading the EXAMPLE file");
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
        println!("\tINCORRECT: Expected: [{}] but got [{}]", example_result, result);
        false
    } else {
        println!("\tCORRECT");
        true
    }
}

fn part1(input: &str) -> u32 {
    let mut cargo = Vec::<Vec<char>>::new();
    cargo.resize(9, Vec::<char>::new());

    let (cargo_plot, instructions) = input.split_once("\n\n").unwrap();

    cargo_plot
        .lines()
        .flat_map(|line| {
            line.chars()
                .collect::<Vec<char>>() // make a copy as a vector
                .chunks(4) // take chunks of chars
                .enumerate()
                .filter(|(_i, chunk)| (chunk[1] as i32) - (b'A' as i32) > 0)
                .map(|(i, chunk)| (i, chunk[1]))
                .collect::<Vec<(_, char)>>()
        })
        .rev()
        .for_each(|(i, c)| cargo[i].push(c));

    instructions
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let instr = line
                .split(" ")
                .filter_map(|pos_num| pos_num.parse::<u32>().ok())
                .map(|n| n - 1)
                .collect::<Vec<u32>>();

            let cargo_size = cargo[instr[1] as usize].len();
            let mut vals = cargo[instr[1] as usize].split_off(cargo_size - (instr[0] as usize) - 1);
            vals.reverse();
            cargo[instr[2] as usize].append(&mut vals);
        });

    let top: String = cargo
        .iter()
        .filter_map(|stack| stack.last())
        .map(|n| *n)
        .collect();

    println!("{:?}", top);

    0
}

fn part2(input: &str) -> u32 {
    let mut cargo = Vec::<Vec<char>>::new();
    cargo.resize(9, Vec::<char>::new());

    let (cargo_plot, instructions) = input.split_once("\n\n").unwrap();

    cargo_plot
        .lines()
        .flat_map(|line| {
            line.chars()
                .collect::<Vec<char>>() // make a copy as a vector
                .chunks(4) // take chunks of chars
                .enumerate()
                .filter(|(_i, chunk)| (chunk[1] as i32) - (b'A' as i32) > 0)
                .map(|(i, chunk)| (i, chunk[1]))
                .collect::<Vec<(_, char)>>()
        })
        .rev()
        .for_each(|(i, c)| cargo[i].push(c));

    instructions
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let instr = line
                .split(" ")
                .filter_map(|pos_num| pos_num.parse::<u32>().ok())
                .map(|n| n - 1)
                .collect::<Vec<u32>>();

            let cargo_size = cargo[instr[1] as usize].len();
            let mut vals = cargo[instr[1] as usize].split_off(cargo_size - (instr[0] as usize) - 1);
            // vals.reverse();
            cargo[instr[2] as usize].append(&mut vals);
        });

    let top: String = cargo
        .iter()
        .filter_map(|stack| stack.last())
        .map(|n| *n)
        .collect();

    println!("{:?}", top);

    0
}