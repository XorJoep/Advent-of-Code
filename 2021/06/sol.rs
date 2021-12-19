use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
    5934;
    let expect_result_part2 = 
    26984457539;
    
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

fn execute_part(part_fn: fn(&str) -> u64, input: &str, example_result: u64) -> bool {
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

fn breed_fish(states: &mut Vec<u64>, size: usize, days: usize) {
    (0..days).for_each(|day| {
        let base_vector = day % size;
        states[(base_vector + 7) % size] += states[base_vector];
    });
}


fn part1(input: &str) -> u64 {
    let size = 9;
    let mut states = vec![0; size];

    // Set initiale states
    input
        .split(",")
        .map(|c| c.parse::<usize>().expect("not integer"))
        .for_each(|d| states[d] += 1);

    // Breed
    breed_fish(&mut states, size, 80);

    // Get sum of fish
    states.iter().sum()
}


fn part2(input: &str) -> u64 {
    let size = 9;
    let mut states = vec![0; size];

    // Set initiale states
    input
        .split(",")
        .map(|c| c.parse::<usize>().expect("not integer"))
        .for_each(|d| states[d] += 1);

    // Breed
    breed_fish(&mut states, size, 256);

    // Get sum of fish
    states.iter().sum()
}
