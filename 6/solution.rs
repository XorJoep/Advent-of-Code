use std::fs;
use std::time::Instant;
use itertools::Itertools;
// use std::collections::hash_map::Entry;
fn main() {
    // let filename = "input";
    let filename = "ex_input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&contents));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&contents));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(input: &str) -> usize {
    let vec: Vec<_> = input
        .split("\n\n")
        .map(|s| s.bytes()
            .filter(|b| *b!=b'\n')
            .collect::<Vec<u8>>())
        .collect();

    let mut sum = 0;
    for s in vec.iter() {
        let mut s2 = s.clone();
        s2.sort();
        s2.dedup();
        sum += s2.len();
    }
    sum
}

fn part2(input: &str) -> usize {
    input
            .split("\n\n")
            .map(|s| s.chars()
                .filter(|&b| b != '\n')
                .unique()
                .map(|answer| s.chars().filter(|&c| c == answer).count())
                .filter(|&count| count == s.lines().count())
            .count())
            .sum::<usize>()
}
