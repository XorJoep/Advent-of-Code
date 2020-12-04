use std::fs;
use std::time::Instant;

const PP_REQS: [&'static str; 7]  = 
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

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

fn part1(input: &str) -> usize {
    return input
        .split("\n\n")
        .filter(|passport| {
                let values: Vec<String> = passport
                                        .split_whitespace()
                                        .map(|s| s
                                                .chars()
                                                .take(3)
                                                .collect())
                                        .collect();
                return PP_REQS.iter().all(|val| values.contains(&val.to_string()))
        })
        .count();
}

fn part2(input: &str) -> usize {
    return input
        .split("\n\n")
        .filter(|passport| 
            passport
            .split_whitespace()
            .map(|s| s.split(":").collect::<Vec<&str>>())
            .filter(|val| 
                match val[0] {
                    "byr" => (val[1].parse::<usize>().unwrap() >= 1920 && val[1].parse::<usize>().unwrap() <= 2002),
                    "iyr" => (val[1].parse::<usize>().unwrap() >= 2010 && val[1].parse::<usize>().unwrap() <= 2020),
                    "eyr" => (val[1].parse::<usize>().unwrap() >= 2020 && val[1].parse::<usize>().unwrap() <= 2030),
                    "hgt" => {
                        if val[1].ends_with("in") {
                            let size = val[1].chars().take(val[1].len()-2).collect::<String>().parse::<usize>().unwrap();
                            size >= 59 && size <= 76
                        }
                        else if val[1].ends_with("cm") {
                            let size = val[1].chars().take(val[1].len()-2).collect::<String>().parse::<usize>().unwrap();
                            size >= 150 && size <= 193
                        }
                        else {
                            false
                        }
                    }
                    "hcl" => val[1].as_bytes()[0] == b'#' 
                          && val[1].bytes().skip(1).all(|b| (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f')),
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val[1]),
                    "pid" => val[1].bytes().all(|b| (b>= b'0' && b <= b'9' )) && val[1].len()==9,
                    _ => false,
                }
            ).count() == 7
        )
        .count();
}

