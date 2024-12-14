use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 143;
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
    let (rules, page_numbers) = input.split_once("\n\n").unwrap();

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    rules
        .lines()
        .filter_map(|l| l.split_once("|"))
        .for_each(|(l, r)| {
            let (l, r) = (l.parse().unwrap(), r.parse().unwrap());
            rules_map.entry(l).or_insert_with(Vec::new).push(r);
        });

    page_numbers.lines().filter_map(|line|{
        let pages: Vec<u32> = line.split(",").map(|n| n.parse().expect("?")).collect();

        let is_ordered = pages.iter().enumerate().all(|(i, p)| {
            if let Some(rule) = rules_map.get(p) {
                rule.iter().all(|r| !&pages[0..i].contains(r))
            } else {
                true
            }
        });

        if is_ordered {
            Some(pages)
        } else {None}
    }).map(|pages| pages[pages.len()/2]).sum()
}

fn part2(input: &str) -> u32 {
    let (rules, page_numbers) = input.split_once("\n\n").unwrap();

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    rules
        .lines()
        .filter_map(|l| l.split_once("|"))
        .for_each(|(l, r)| {
            let (l, r) = (l.parse().unwrap(), r.parse().unwrap());
            rules_map.entry(l).or_insert_with(Vec::new).push(r);
        });

    page_numbers.lines().filter_map(|line|{
        let pages: Vec<u32> = line.split(",").map(|n| n.parse().expect("?")).collect();

        let is_ordered = pages.iter().enumerate().all(|(i, p)| {
            if let Some(rule) = rules_map.get(p) {
                rule.iter().all(|r| !&pages[0..i].contains(r))
            } else {
                true
            }
        });

        if is_ordered {
            None
        } else {Some(pages)}
    }).count() as u32
}
