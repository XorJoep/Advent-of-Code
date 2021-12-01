use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    // let filename = "ex_input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&contents));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&contents));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .count())

}

fn part2(input: &str) -> usize {
    input
        .lines()
        .count()
}
