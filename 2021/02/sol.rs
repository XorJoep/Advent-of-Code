use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    //let filename = "ex_input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&contents));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&contents));
    println!("Finished after {:?}", start.elapsed());
}

fn part1(input: &str) -> i32 {
    let (hor, vert) = input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(instr, val)| (instr, val.parse::<i32>().expect("not integer")))
        .fold((0, 0), |(hor, vert), (instr, val)| 
            match instr {
                "forward" => (hor + val, vert),
                "down" => (hor, vert + val),
                "up" => (hor, vert - val),
                _ => (hor, vert)
            }
        );
        
    hor * vert
}

fn part2(input: &str) -> i32 {
    let (hor, vert, _aim) = input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(instr, val)| (instr, val.parse::<i32>().expect("not integer")))
        .fold((0, 0, 0), |(hor, vert, aim), (instr, val)| 
            match instr {
                "forward" => (hor + val, vert + aim * val, aim),
                "down" => (hor, vert, aim + val),
                "up" => (hor, vert, aim - val),
                _ => (hor, vert, aim)
            }
        );

    hor * vert
}
