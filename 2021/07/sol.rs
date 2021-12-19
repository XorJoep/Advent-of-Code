use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
        37;
    let expect_result_part2 = 
        170;
    
    let filename_example = "ex_input";
    let filename= "input";
    
    let contents_example = fs::read_to_string(filename_example).expect("Something went wrong reading the example file");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the example file");

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

    let succes;
    if example_result == 0 {
        succes = true;
    }
    else if example_result != 0 && result != example_result {
        println!("\tINCORRECT: Expected: [{}] but got [{}]", example_result, result);
        succes = false
    }
    else {
        println!("\tExample CORRECT");
        succes = true;
    }

    succes
}

fn abs_diff(a: u32, b: u32) -> u32 {
    if a > b {a - b}
    else {b - a}
}


fn part1(input: &str) -> u32 {
    let mut positions: Vec<u32> = input
        .split(",")
        .map(|c| c.parse::<u32>().expect("not integer"))
        .collect();
    let n_pos = positions.iter().count();
    positions.sort();
    let optimal_position = positions[n_pos / 2];

    positions
        .iter()
        .map(|pos| abs_diff(*pos, optimal_position))
        .sum()
}


fn part2(input: &str) -> u32 {
    let positions: Vec<u32> = input
        .split(",")
        .map(|c| c.parse::<u32>().expect("not integer"))
        .collect();
    let n_pos = positions.iter().count() as f32;
    let sum_pos = positions.iter().sum::<u32>();
    let optimal_position = (1.0 * sum_pos as f32 / n_pos).floor() as u32;

    let fuel_floor = positions
        .iter()
        .map(|pos| {
            let diff = abs_diff(*pos, optimal_position);
            diff * (diff+1) / 2
        })
        .sum::<u32>();

    let fuel_ceil = positions
        .iter()
        .map(|pos| {
            let diff = abs_diff(*pos, optimal_position + 1);
            diff * (diff+1) / 2
        })
        .sum::<u32>();
    println!("\t-----\n\tfloor: {}, ceil: {}\n\t-----", fuel_floor, fuel_ceil);

    fuel_floor
}
