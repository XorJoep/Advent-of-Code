#![feature(str_split_once)]

use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let ilist = instruction_list(&contents);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&ilist));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&ilist));
    println!("Finished after {:?}", start.elapsed());
}

fn instruction_list(input: &str) -> Vec<(&str, i32)>{
    input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(instr, val)| (instr, val.parse().expect("not integer")))
        .collect::<Vec<(&str, i32)>>()
}

fn execute((instr, val): (&str, i32), ip: &mut usize, acc: &mut i32) {
    match instr {
        "nop" => *ip += 1,
        "acc" => { *acc += val; *ip += 1},
        "jmp" => *ip = (*ip as i32 + val) as usize,
        _ => *ip +=1,
    }
}

fn run(ilist: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut acc = 0;
    let mut ip = 0;
    let mut ip_seen = vec![false; ilist.len()];

    loop {
        ip_seen[ip] = true;
        execute(ilist[ip], &mut ip, &mut acc);
        if ip == ilist.len() {return (acc, true)}
        else if ip_seen[ip] {return (acc, false)}
    }
}

fn part1(ilist: &Vec<(&str, i32)>) -> i32 {
    run(ilist).0
}

fn part2(ilist: &Vec<(&str, i32)>) -> i32 {
    (0..ilist.len()).filter_map(|ip| {
            let new_instr = 
                match ilist[ip].0 {
                    "nop" => ("jmp", ilist[ip].1),
                    "jmp" => ("nop", ilist[ip].1),
                    _ => return None,
                };
            let mut new_code = ilist.clone();
            new_code[ip] = new_instr;

            let (acc, terminated) = run(&new_code);

            if terminated {Some(acc)}
            else {None}
        }).next().unwrap()
}
