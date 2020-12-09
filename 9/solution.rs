#![feature(str_split_once)]

use std::fs;
use std::time::Instant;
use itertools::Itertools;

const PREAMBLE_SIZE: usize = 25;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let ilist = parse_input(&contents);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&ilist));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&ilist));
    println!("Finished after {:?}", start.elapsed());
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().expect("not integer")).collect()
}

fn part1(ilist: &Vec<usize>) -> usize {
    let index = 
    (0..ilist.len()-PREAMBLE_SIZE).take_while(|&ii| {
        ilist[ii..ii+PREAMBLE_SIZE]
            .iter()
            .tuple_combinations::<(&usize, &usize)>()
            .any(|(v1, v2)|
                v1+v2 == ilist[ii+PREAMBLE_SIZE])
        }
    ).count();
    ilist[index+PREAMBLE_SIZE]
}

fn part2(ilist: &Vec<usize>) -> usize {
    let num_to_find = part1(ilist);

    for offset in 0..ilist.len() {
        let mut sum = 0;
        for size in 0..ilist.len()-offset {
            sum += ilist[offset+size];

            if sum == num_to_find {
                let m1 = ilist[offset..offset+size].iter().min().unwrap();
                let m2 = ilist[offset..offset+size].iter().max().unwrap();
                return m1 + m2
            }
            else if sum >= num_to_find {
                break;
            }
        }
    }
    return 0
}