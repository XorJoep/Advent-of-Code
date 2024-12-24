use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let expect_result_part1 = 55312;
    let expect_result_part2 = 0;

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
    let mut nums: Vec<usize> = input.split(" ").map(|c| c.parse().unwrap()).collect();

    for _ in 0..25 {
        nums = nums
            .iter()
            .flat_map(|num| {
                if *num == 0 {
                    vec![1]
                } else if num.to_string().len() % 2 == 0 {
                    let str_num = num.to_string();
                    let (left, right) = str_num.split_at(str_num.len() / 2);
                    vec![
                        left.parse::<usize>().unwrap(),
                        right.parse::<usize>().unwrap(),
                    ]
                } else {
                    vec![num * 2024]
                }
            })
            .collect();
    }

    nums.len() as u64
}

fn part2(input: &str) -> u64 {
    let mut nums: HashMap<u64, u64> =
        input
            .split(" ")
            .map(|c| c.parse().unwrap())
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(item).or_insert(0) += 1;
                acc
            });

    for _ in 0..75 {
        nums = nums.into_iter().fold(HashMap::new(), |mut acc, (num, count)| {
                if num == 0 {
                    *acc.entry(1).or_insert(0) += count;
                } else if num.to_string().len() % 2 == 0 {
                    let str_num = num.to_string();
                    let (left, right) = str_num.split_at(str_num.len() / 2);
                    *acc.entry(left.parse::<u64>().unwrap()).or_insert(0) += count;
                    *acc.entry(right.parse::<u64>().unwrap()).or_insert(0) += count;
                } else {
                    *acc.entry(num * 2024).or_insert(0) += count;
                }
                acc
        });

    }

    nums.values().sum::<u64>() as u64
}
