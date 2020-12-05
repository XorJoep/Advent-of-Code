use std::fs;
use std::time::Instant;
use std::collections::HashSet;
// use std::iter::FromIterator;



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

fn convert(code: &str) -> usize {
    let val = code.chars().map(|c|
        match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => 'X'
        }
    ).collect::<String>();
    usize::from_str_radix(&val, 2).unwrap()
}

fn part1(input: &str) -> usize {
    return input
            .lines()
            .map(|s| s.split_at(7))
            .map(|(row, col)| {
                convert(row)*8+convert(col)
            })
            .fold(0, |acc, seat_id| acc.max(seat_id)); // 20 minutes
}

fn part2(input: &str) -> usize {
    let seats_taken: HashSet<usize> = 
            input
            .lines()
            .map(|s| s.split_at(7))
            .map(|(row, col)| {
                convert(row)*8+convert(col)
            })
            .collect();
    let seats_available = (0..807 as usize).collect::<HashSet<usize>>();
    let remaining_seats = seats_available.difference(&seats_taken);
    remaining_seats.into_iter().fold(0, |acc, seat| acc.max(*seat))
}