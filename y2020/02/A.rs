use std::fs;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n")
                                 // .map(|s| s.parse().expect("parse error"))
                                 .collect();

    let mut count = 0;

    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        let vals: Vec<&str> = line.split_whitespace().collect();
        let (range, letter, pw) : (&str, char, &str) = (vals[0], vals[1].chars().nth(0).unwrap(), vals[2]);
        let ranges: Vec<u32> = range.split("-").map(|s| s.parse().expect("parse error")).collect();
        let (range_min, range_max) = (ranges[0], ranges[1]);

        let num_found = pw.chars().filter(|&c| (c==letter)).count() as u32;
        if num_found >= range_min && num_found <= range_max {
            count += 1;
            // println!("{}, {}, {}", range, num_found, count);
        }
    }

    println!("{}", count);
    
}