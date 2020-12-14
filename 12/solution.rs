use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let instruction_list = parse_input(&contents);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&instruction_list));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&instruction_list));
    println!("Finished after {:?}", start.elapsed());
}

fn parse_input(input: &str) -> Vec<(u8, i32)> {
    input
        .lines()
        .map(|l| {
            let (l, num) = l.split_at(1);
            (l.bytes().nth(0).unwrap(), num.parse::<i32>().unwrap())
        })
        .collect::<Vec<(u8, i32)>>()
}

fn py_mod(a: i32, b: i32) -> i32 {
    let res = a % b;
    if res < 0 {
        return res + b;
    }
    res
}

fn part1(instruction_list: &Vec<(u8, i32)>) -> i32 {
    let mut dir: i32 = 1;
    let mut pos: (i32, i32) = (0, 0);
    for (i, amount) in instruction_list {
        match i {
            b'N' => pos.1 += amount,
            b'S' => pos.1 -= amount,
            b'E' => pos.0 += amount,
            b'W' => pos.0 -= amount,
            b'L' => dir = py_mod(dir - amount/90, 4),
            b'R' => dir = py_mod(dir + amount/90, 4),
            b'F' => match dir {
                0 => pos.1 += amount,
                1 => pos.0 += amount,
                2 => pos.1 -= amount,
                3 => pos.0 -= amount,
                _ => {},
            },
            _ => {},            
        }
    }
    pos.0.abs() + pos.1.abs()

}

fn part2(instruction_list: &Vec<(u8, i32)>) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut wp: (i32, i32) = (10, 1);
    for (i, amount) in instruction_list {
        match i {
            b'N' => wp.1 += amount,
            b'S' => wp.1 -= amount,
            b'E' => wp.0 += amount,
            b'W' => wp.0 -= amount,
            b'L' => 
                wp = match amount {
                    90 => (-wp.1, wp.0),
                    180 => (-wp.0, -wp.1),
                    270 => (wp.1, -wp.0),
                    _ => wp,
                },
            b'R' => 
                wp = match amount {
                    90 => (wp.1, -wp.0),
                    180 => (-wp.0, -wp.1),
                    270 => (-wp.1, wp.0),
                    _ => wp,
                },
            b'F' => {
                pos.0 += wp.0 * amount; 
                pos.1 += wp.1 * amount;
            },
            _ => {},            
        }
    }
    pos.0.abs() + pos.1.abs()
}