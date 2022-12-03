use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
        2;
    let expect_result_part2 = 
        1;
    
    let filename_example = "ex_input";
    let filename = "input";
    
    let contents_example = fs::read_to_string(filename_example).expect("Erro reading the EXAMPLE file");
    let contents = fs::read_to_string(filename).expect("ERROR reading the INPUT file");

    let mut succes;
    println!("Part 1 - Example");
    succes = execute_part(part1, &contents_example, expect_result_part1);
    println!("Part 1");
    succes &= execute_part(part1, &contents, 0);
    println!("Part 2 - Example");
    succes &= execute_part(part2, &contents_example, expect_result_part2);
    println!("Part 2");
    execute_part(part2, &contents, 0);
}

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32) -> bool {
    let start = Instant::now();
    let result = part_fn(&input);

    println!("\tFinished after {:?}", start.elapsed());
    println!("\tSolution found: {}", result);

    if example_result == 0 {
        println!("\tSkipping check");
        true
    }
    else if result != example_result {
        println!("\tINCORRECT: Expected: [{}] but got [{}]", example_result, result);
        false
    }
    else {
        println!("\tCORRECT");
        true
    }
}


fn part1(input: &str) -> u32 {
    input
        .lines()
        .count() as u32
}


fn part2(input: &str) -> u32 {
    input
        .lines()
        .count() as u32
}
