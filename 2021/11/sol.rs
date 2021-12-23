use std::fs;
use std::time::Instant;

mod matrix;

fn main() {
    let expect_result_part1 = 
    1656;
    let expect_result_part2 = 
    195;
    
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
    let mut matrix = matrix::Matrix::new(input);
    let steps = 100;
    let mut flashes = 0;
    for _ in 0..steps {
        matrix.incr_all();
        flashes += matrix.reset_all();
    }
    flashes as u32
}


fn part2(input: &str) -> u32 {
    let mut matrix = matrix::Matrix::new(input);
    let matrix_size = matrix.size();
    (1..).find(|_| {
        matrix.incr_all();
        matrix_size == matrix.reset_all()
    })
    .unwrap()
}
