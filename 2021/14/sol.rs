use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let expect_result_part1 = 
    1588;
    let expect_result_part2 = 
    2188189693529;
    
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

fn key_to_keys(mut key_in: String, insert_val: char) -> (String, String) {
    let mut part = key_in.split_off(1);
    key_in.push(insert_val);
    part.insert(0, insert_val);
    (key_in, part)
}

fn polymer_length(polymer_buckets: &HashMap<String, u64>) -> u64 {
    polymer_buckets.iter().fold(1, |acc, (_, amount)| acc + amount)
}

fn calc_polymers(input: &str, steps: usize) -> u64 {
    let first_polymer: Vec<char> = 
    input
        .lines()
        .take(1)
        .flat_map(|line| 
            line
                .chars()
                .collect::<Vec<char>>()
            )
        .collect::<Vec<char>>();

    // convert the first polymere into a hashmap of tuples
    let mut polymer_buckets = HashMap::new();        
    first_polymer
        .windows(2)
        .for_each(|tuple_pair|
            *polymer_buckets
                .entry(tuple_pair.iter().collect::<String>())
                .or_insert(0) += 1
        );
    println!("Starting Polymer: {:?}", polymer_buckets);

    // convert the rules into a mapping of rules
    let rules: HashMap<String, _ > = 
    input
        .lines()
        .skip(1)
        .filter_map(|rule| rule.split_once(" -> "))
        .map(|(rule, addition)| (rule.chars().take(2).collect::<String>(), addition.chars().next().unwrap()))
        .map(|(rule, addition)| (rule.clone(), key_to_keys(rule, addition)))
        .collect();
    
    println!("pol buckets: {:?}", polymer_buckets);
    
    (1..steps+1).for_each(|step| {
        let mut new_polymer_buckets = HashMap::new();
        rules.iter().for_each(|(rule_key, rule_val)| {
            let addition_amount = *polymer_buckets.entry(rule_key.clone()).or_insert(0);
            let (key1, key2) = rule_val;
            *new_polymer_buckets.entry(key1.clone()).or_insert(0) += addition_amount;
            *new_polymer_buckets.entry(key2.clone()).or_insert(0) += addition_amount;
        });
        polymer_buckets = new_polymer_buckets.clone();
        // println!("[{}] size: {} - buckets: {:?}", step, polymer_length(&polymer_buckets), polymer_buckets);
    });

    let mut occurrences = HashMap::new();  
    polymer_buckets
        .iter()
        .for_each(|(polymer, amount)| 
            polymer
                .as_bytes()
                .iter()
                .for_each(|c| {
                    *occurrences
                        .entry(c)
                        .or_insert(0) += amount;
                })
            );
    
    let (max_key, max_amount) = occurrences.iter().max_by_key(|entry| entry.1).unwrap();
    let (min_key, min_amount) = occurrences.iter().min_by_key(|entry| entry.1).unwrap();
    println!("min: {:?}: {} - max: {:?}: {}", **min_key as char, min_amount, **max_key as char, max_amount);

    (max_amount+1)/2-min_amount/2

}

fn part1(input: &str) -> u64 {
    let steps = 10;
    calc_polymers(input, steps)
}


fn part2(input: &str) -> u64 {
    let steps = 40;
    calc_polymers(input, steps)
}
