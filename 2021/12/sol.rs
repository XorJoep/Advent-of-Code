use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let expect_result_part1 = 
        10;
    let expect_result_part2 = 
        36;
    
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

fn add_cave<'a>(caves: &mut HashMap<&'a str, Vec<&'a str>>, c1: &'a str, c2: &'a str) {
    if caves.contains_key(c1) {
        caves.get_mut(c1).unwrap().push(c2)
    }
    else {
        caves.insert(c1, vec![c2]);
    }

    if caves.contains_key(c2) {
        caves.get_mut(c2).unwrap().push(c1)
    }
    else {
        caves.insert(c2, vec![c1]);
    }
}

fn dive_deeper<'a>(caves: &HashMap<&'a str, Vec<&'a str>>, chains: &mut Vec<Vec<&'a str>>, chain: &mut Vec<&'a str>, got_time: bool) {
    let current_cave = chain.last().unwrap();
    for next_cave in &caves[current_cave] {
        if *next_cave == "start" { continue}

        let seen_cave = chain.contains(&next_cave);
        let is_big_cave = next_cave.chars().nth(0).unwrap().is_ascii_uppercase();
        if !seen_cave || is_big_cave || got_time {
            chain.push(next_cave);
            if *next_cave == "end" {
                chains.push(chain.clone());
            }
            else {
                if seen_cave && !is_big_cave && got_time {
                    dive_deeper(caves, chains, chain, false);
                }
                else {
                    dive_deeper(caves, chains, chain, got_time);
                }
            }
            chain.pop();
        }
    }
}


fn part1(input: &str) -> u32 {
    let mut cave_map = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.split_once("-"))
        .for_each(|(c1, c2)| {
            add_cave(&mut cave_map, c1, c2)
        });
    
    let mut chains = Vec::new();
    let mut chain = vec!["start"];

    dive_deeper(&cave_map, &mut chains, &mut chain, false);
    
    chains.iter().count() as u32

}


fn part2(input: &str) -> u32 {
    let mut cave_map = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.split_once("-"))
        .for_each(|(c1, c2)| {
            add_cave(&mut cave_map, c1, c2)
        });
    
    let mut chains = Vec::new();
    let mut chain = vec!["start"];

    dive_deeper(&cave_map, &mut chains, &mut chain, true);

    chains.iter().count() as u32
}
