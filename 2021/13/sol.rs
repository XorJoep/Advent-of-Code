use std::fs;
use std::time::Instant;
use std::cmp::max;

mod matrix;

fn main() {
    let expect_result_part1 = 
        17;
    let expect_result_part2 = 
        1;
    
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


fn part1(input: &str) -> u32 {
    let (max_r, max_c) = 
    input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .fold((0, 0), |(x, y), (ax, ay)| (max(x, ax), max(y, ay)));

    let mut matrix = matrix::Matrix::new(max_r+1, max_c+1);
    input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .for_each(|(x, y)| matrix.activate(x, y));
    
        // matrix.print();
    input
        .lines()
        .filter_map(|line| line.split_once("="))
        .map(|(horizontal, foldline)| (horizontal.chars().last().unwrap() == 'x', foldline.parse().unwrap()))
        .take(1)
        .for_each(|(horizontal, foldline)| matrix.fold(foldline, horizontal));
        // matrix.print();
    matrix.visible_dots()
}


fn part2(input: &str) -> u32 {
    let (max_r, max_c) = 
    input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .fold((0, 0), |(x, y), (ax, ay)| (max(x, ax), max(y, ay)));

    let mut matrix = matrix::Matrix::new(max_r+1, max_c+2);
    input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .for_each(|(x, y)| matrix.activate(x, y));
    
    input
        .lines()
        .filter_map(|line| line.split_once("="))
        .map(|(horizontal, foldline)| (horizontal.chars().last().unwrap() == 'x', foldline.parse().unwrap()))
        .for_each(|(horizontal, foldline)| {matrix.fold(foldline, horizontal);});
    matrix.print();
    1
}
