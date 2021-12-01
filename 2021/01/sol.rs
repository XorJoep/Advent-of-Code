use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    //let filename = "ex_input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data: Vec<i32> = contents
        .lines()
        .map(|s| s.parse().expect("parse error"))
                                .collect();

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&data));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&data));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(data: &Vec<i32>) -> usize {
    data
        .iter()
        .zip(data
                .iter()
                .skip(1)
            )
        .filter(|(current, next)| current < next)
        .count()
}

fn part2(data: &Vec<i32>) -> usize {
    data
        .iter()
        .zip(data
                .iter()
                .skip(3)
            )
        .filter(|(current, next)| current < next)
        .count()
}
