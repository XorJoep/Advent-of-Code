use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 11;
    let expect_result_part2 = 31;

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

fn part1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(left, right)| {
            (
                left.parse::<i32>().expect("not integer"),
                right.parse::<i32>().expect("not integer"),
            )
        })
        .collect();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs() as u32)
        .sum()
}

use std::collections::HashMap;
fn part2(data: &str) -> u32 {
    let mut entries = HashMap::new();

    data.lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(_, right)| right.parse::<u32>().expect("not integer"))
        .for_each(|val| *entries.entry(val).or_insert(0) += 1);

    data.lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(left, _)| {
            let val = left.parse::<u32>().expect("not integer");
            entries.get(&val).unwrap_or(&0) * val
        })
        .sum()
}
