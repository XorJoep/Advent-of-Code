use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let expect_result_part1 = 
        26397;
    let expect_result_part2 = 
        288957;
    
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

fn part1(input: &str) -> u64 {
    let points = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    
    let pairs = HashSet::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    input
        .lines()
        .map(|line| {
            let mut buff = Vec::new();
            line
                .chars()
                .find_map(|c| {
                    match c {
                        '(' | '[' | '{' | '<' => {buff.push(c); None}
                        ')' | ']' | '}' | '>' => {
                            if !pairs.contains(&(buff.pop().unwrap_or('_'), c)) {
                                points.get(&c)
                            }
                            else {None}
                        }
                        _ => (None),
                    }
                }).unwrap_or(&0)
        })
        .sum()
}


fn part2(input: &str) -> u64 {
    let points = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let pairs = HashSet::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut scores = input
        .lines()
        .filter_map(|line| {
            let mut buff = Vec::new();
            if line
                .chars()
                .find(|&c| {
                    match c {
                        '(' | '[' | '{' | '<' => {buff.push(c); false},
                        ')' | ']' | '}' | '>' => {
                            if pairs.contains(&(buff.pop().unwrap_or('_'), c)) {false}
                            else {true} // break on corrupted line -> else continue
                        }
                        _ => (false),
                    }
                })
                .is_some() {
                    None // -> Had to break, so line is corrupted -> return none
                }
            else { 
                Some(buff.iter().rfold(0, |acc, val| acc*5 + points.get(val).expect("HUH")))
            }
        })
        .collect::<Vec<u64>>();
    
    // get the middle value
    scores.sort();
    scores[scores.len() / 2]
}
