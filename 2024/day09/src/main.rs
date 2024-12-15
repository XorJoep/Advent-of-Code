use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 1928;
    let expect_result_part2 = 1;

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

fn execute_part(part_fn: fn(&str) -> u64, input: &str, example_result: u64) -> bool {
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

fn part1(input: &str) -> u64 {
    let mut buffer: Vec<Option<usize>> = Vec::new();
    input
        .lines()
        .next()
        .expect("")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .for_each(|(i,c)| {
            if i%2 == 0 {
                buffer.extend(std::iter::repeat(Some(i/2)).take(c))
            } else {
                buffer.extend(std::iter::repeat(None).take(c))
            }
        });
    
    let mut front_ptr = 0 as i32;
    let mut back_ptr = buffer.len() as i32 - 1;

    while front_ptr < back_ptr {
        if let c = buffer[back_ptr as usize] {
            while front_ptr < back_ptr {
                if !buffer[front_ptr as usize].is_some() {
                    buffer[front_ptr as usize] = c;
                    buffer[back_ptr as usize] = None;
                    back_ptr -= 1;
                    break;
                } else {
                    front_ptr += 1;
                }
            }
        } else {
            back_ptr -= 1;
        }
    }

    buffer.iter().filter_map(|&c| c).enumerate().map(|(i,c)| (i*c) as u64).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().count() as u64
}
