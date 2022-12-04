use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 2;
    let expect_result_part2 = 4;

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

fn to_range(input: &str) -> (u32, u32) {
    input
        .split_once("-")
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .expect("no - found")
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(elf1, elf2)| (to_range(elf1), to_range(elf2)))
        .filter(|((s1, e1), (s2, e2))| (s1 >= s2) && (e2 >= e1) || (s2 >= s1) && (e1 >= e2))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(elf1, elf2)| (to_range(elf1), to_range(elf2)))
        .filter(|((s1, e1), (s2, e2))| !(e1 < s2 || e2 < s1))
        .count() as u32
}