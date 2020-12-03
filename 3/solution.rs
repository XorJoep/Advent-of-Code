use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&contents));
    println!("Finished after {:?}", start.elapsed());   

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&contents));
    println!("Finished after {:?}", start.elapsed());   
    
    let start = Instant::now();
    println!("Solution for PART 2_1: {}", part2_1(&contents));
    println!("Finished after {:?}", start.elapsed());  
    
    let start = Instant::now();
    println!("Solution for PART 2_2: {}", part2_2(&contents));
    println!("Finished after {:?}", start.elapsed());  
}

fn count_trees(input: &str, right: usize, down: usize) -> usize {
    return input
        .lines()
        .enumerate()
        .filter(|&(i, l)| 
            i % down == 0 &&
            l.chars()
             .nth(i*right/down % l.len())
             .unwrap() == '#')
        .count();
}

fn part1(input: &str) -> usize {
    return count_trees(input, 3, 1);
}

fn part2(input: &str) -> usize {
    return count_trees(input, 1, 1)
           * count_trees(input, 3, 1)
           * count_trees(input, 5, 1)
           * count_trees(input, 7, 1)
           * count_trees(input, 1, 2);
}

fn part2_1(input: &str) -> usize {
    return  [(1,1),
             (3,1),
             (5,1),
             (7,1),
             (1,2)].iter()
                   .fold(1, |acc, &(right, down)| acc * count_trees(input, right, down));
}

fn part2_2(input: &str) -> usize {
    return  [(1,1),
             (3,1),
             (5,1),
             (7,1),
             (1,2)].iter()
                   .map(|&(right, down)| count_trees(input, right, down))
                   .product();
}