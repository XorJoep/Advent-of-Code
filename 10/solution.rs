use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let numlist = parse_input(&contents);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&numlist));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&numlist));
    println!("Finished after {:?}", start.elapsed());
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut ret = input.lines().map(|l| l.parse().expect("not integer")).collect::<Vec<usize>>();
    ret.push(0);
    ret.push(ret.iter().max().unwrap() + 3);
    ret.sort_unstable();
    ret
}

fn part1(numlist: &Vec<usize>) -> usize {
    let mut diff = vec![0;4];
    for index in 0..numlist.len()-1 {
        diff[numlist[index+1] - numlist[index]] += 1;
    }
    diff[1] * diff[3]
}

fn part2(numlist: &Vec<usize>) -> usize {
    let mut ways = vec![0; numlist.len()];
    ways[0] = 1;
    for n in 0..numlist.len() {
        for size in 1..4 {
            if (n as i32)-(size as i32) < 0 {break}
            if numlist[n] - numlist[n-size] <= 3 {ways[n] += ways[n-size]}
        }
    }
    // println!("{:?}", ways);
    ways[numlist.len()-1]
}