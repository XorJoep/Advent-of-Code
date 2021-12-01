#![feature(str_split_once)]

use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    // let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&contents));
    println!("Finished after {:?}", start.elapsed());   

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&contents));
    println!("Finished after {:?}", start.elapsed());   
}

fn part1(input: &str) -> usize {
    return input
        .split("\n\n")
        .filter(|passport| {
                passport
                    .split_whitespace()
                    .filter_map(|s| s.split_once(":"))
                    .filter(|(key, _val)| 
                        "cid" != *key)
                    .count() >= 7
        })
        .count();
}

fn part2(input: &str) -> usize {
    return input
        .split("\n\n")
        .filter(|passport| 
            passport
            .split_whitespace()
            .filter_map(|s| s.split_once(":"))
            .filter(|&(key, val)| 
                match key {
                    "byr" => (val.parse::<usize>().unwrap() >= 1920 && val.parse::<usize>().unwrap() <= 2002),
                    "iyr" => (val.parse::<usize>().unwrap() >= 2010 && val.parse::<usize>().unwrap() <= 2020),
                    "eyr" => (val.parse::<usize>().unwrap() >= 2020 && val.parse::<usize>().unwrap() <= 2030),
                    "hgt" => {
                        if val.ends_with("in") {
                            let size = val.chars().take(val.len()-2).collect::<String>().parse::<usize>().unwrap();
                            size >= 59 && size <= 76
                        }
                        else if val.ends_with("cm") {
                            let size = val.chars().take(val.len()-2).collect::<String>().parse::<usize>().unwrap();
                            size >= 150 && size <= 193
                        }
                        else {
                            false
                        }
                    }
                    "hcl" => val.as_bytes()[0] == b'#' 
                          && val.bytes().skip(1).all(|b| b.is_ascii_hexdigit()),
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val),
                    "pid" => val.bytes().all(|b| b.is_ascii_digit()) && val.len()==9,
                    _ => false,
                }
            ).count() == 7
        )
        .count();
}

