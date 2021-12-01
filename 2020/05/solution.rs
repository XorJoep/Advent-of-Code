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

fn convert(code: &str) -> usize {
    code
        .bytes().rev()
        .enumerate()
        .fold(0, |acc, (i, b)| (((b==b'B' || b == b'R') as usize) << i) + acc)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_at(7))
        .map(|(row, col)| convert(row) * 8 + convert(col))
        .max().unwrap()
}

fn part2(input: &str) -> usize {
    let seats_taken: Vec<usize> = input
        .lines()
        .map(|l| l.split_at(7))
        .map(|(row, col)| convert(row) * 8 + convert(col))
        .collect();

    let min_id = *seats_taken.iter().min().unwrap();
    let max_id = *seats_taken.iter().max().unwrap();

    let sum_normal = max_id/2 * (max_id+1) - (min_id-1)/2 * (min_id);
    let sum_id = seats_taken.iter().sum::<usize>();
    sum_normal - sum_id
}
