#![feature(str_split_once)]

use std::fs;
use std::time::Instant;
use std::collections::HashMap;

struct Bag {
    name: String,
    amount: usize,
}

const GOLD_BAG_STRING: &'static str = "shiny gold";

fn main() {
    // let filename = "input";
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let bag_mapping = gen_hashmap(&contents);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&bag_mapping));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&bag_mapping));
    println!("Finished after {:?}", start.elapsed());
}

fn gen_hashmap(input: &str) -> HashMap::<&str, Vec<Bag>>{
    input
        .lines()
        .filter_map(|line| line.split_once(" bags contain "))
        .map(|(bag_top, contents)|
                (bag_top, contents
                            .split(", ")
                            .map(|bag|
                                bag
                                    .split_once(" ")
                                    .map(|(amount, name)|
                                        Bag {
                                            amount: amount.parse::<usize>().unwrap_or(0),
                                            name: name.rsplitn(2, " ").last().unwrap().to_string(),
                                        }
                                    ).unwrap()
                                )
                            .collect::<Vec<Bag>>()
                )
            )
        .collect::<HashMap::<&str, Vec<Bag>>>()
}

/////////////////////////

fn has_gold_bag(mapping: &HashMap::<&str, Vec<Bag>>, bag_name: &str) -> bool {
    mapping
        .get(bag_name).unwrap()
        .iter() // iterate over vector of bags
        .fold(false, |acc, bag| acc || bag.amount != 0 
                                                && (bag.name == GOLD_BAG_STRING 
                                                    || has_gold_bag(&mapping, &bag.name)))
}

fn part1(mapping: &HashMap::<&str, Vec<Bag>>) -> usize {
    mapping
        .keys()
        .filter(|bag| 
            has_gold_bag(&mapping, bag))
        .count()
}

/////////////////////////

fn amount_of_bags_within(mapping: &HashMap::<&str, Vec<Bag>>, bag_name: &str) -> usize {
    mapping
        .get(bag_name).unwrap_or(&Vec::<Bag>::new())
        .iter()
        .fold(0, |acc, bag| acc 
            + bag.amount 
            + (bag.amount) * amount_of_bags_within(&mapping, &bag.name))
}


fn part2(mapping: &HashMap::<&str, Vec<Bag>>) -> usize {
    amount_of_bags_within(&mapping, GOLD_BAG_STRING)
}
