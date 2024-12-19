use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 1928;
    let expect_result_part2 = 2858;

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
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .for_each(|(i, c)| {
            if i % 2 == 0 {
                buffer.extend(std::iter::repeat(Some(i / 2)).take(c))
            } else {
                buffer.extend(std::iter::repeat(None).take(c))
            }
        });

    let mut front_ptr = 0 as usize;
    let mut back_ptr = buffer.len() as usize - 1;

    let mut sum = 0;

    while front_ptr <= back_ptr {
        match buffer[front_ptr] {
            Some(x) => {
                sum += x * front_ptr;
                front_ptr += 1
            }
            None => {
                match buffer[back_ptr] {
                    None => {}
                    Some(x) => {
                        sum += x * front_ptr;
                        front_ptr += 1
                    }
                }
                back_ptr -= 1
            }
        }
    }

    sum as u64
}

fn part2(input: &str) -> u64 {
    let mut buffer: Vec<Option<usize>> = Vec::new();
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .for_each(|(i, c)| {
            if i % 2 == 0 {
                buffer.extend(std::iter::repeat(Some(i / 2)).take(c))
            } else {
                buffer.extend(std::iter::repeat(None).take(c))
            }
        });

    let last_num: usize = buffer.iter().rev().find_map(|s| *s).unwrap();

    for i in (0..=last_num).rev() {
        let data_start = buffer
            .iter()
            .position(|c| match c {
                Some(x) => *x == i,
                _ => false,
            })
            .unwrap();

        let data_size = buffer
            .iter()
            .skip(data_start)
            .take_while(|s| match s {
                Some(x) => *x == i,
                _ => false,
            })
            .count();

        let mut counter = 0;
        if let Some(empty_data_spot) = buffer.iter().position(|s| match s {
            Some(_) => {
                counter = 0;
                false
            }
            None => {
                counter += 1;
                if counter == data_size {
                    true
                } else {
                    false
                }
            }
        }) {
            if empty_data_spot <= data_start {
                for j in 0..data_size {
                    buffer[data_start + j] = None;
                    buffer[empty_data_spot + j - data_size + 1] = Some(i);
                }
            }
        }
    }

    buffer
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            Some(x) => i * x,
            _ => 0,
        })
        .sum::<usize>() as u64
}
