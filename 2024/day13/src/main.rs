use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 480;
    let expect_result_part2 = 0;

    let filename_example = "ex_input";
    let filename = "input";

    let contents_example =
        fs::read_to_string(filename_example).expect("Erro reading the EXAMPLE file");
    let contents = fs::read_to_string(filename).expect("ERROR reading the INPUT file");

    let mut succes;
    println!("Part 1 - Example");
    succes = execute_part(part1, &contents_example, expect_result_part1);
    println!("Part 1");
    succes = succes && execute_part(part1, &contents, 0);
    println!("Part 2 - Example");
    succes = succes && execute_part(part2, &contents_example, expect_result_part2);
    println!("Part 2");
    let _ = succes && execute_part(part2, &contents, 0);
}

fn execute_part(part_fn: fn(&str) -> usize, input: &str, example_result: usize) -> bool {
    let start = Instant::now();
    let result = part_fn(&input);

    println!("\tFinished after {:?}", start.elapsed());
    println!("\tSolution found: {}", result);

    if example_result == 0 {
        println!("\tSkipping check");
        true
    } else if result != example_result {
        println!(
            "\tINCORRECT: Expected: [{}] but got [{}]",
            example_result, result
        );
        false
    } else {
        println!("\tCORRECT");
        true
    }
}

use std::str::FromStr;

#[derive(Debug)]
pub struct Claw {
    button_ax: isize,
    button_ay: isize,
    button_bx: isize,
    button_by: isize,
    prize_x: isize,
    prize_y: isize,
}

impl Claw {
    fn solve(&self) -> isize {
        let y_top = self.button_ay * self.prize_x - self.button_ax * self.prize_y;
        let y_bot = self.button_bx * self.button_ay - self.button_ax * self.button_by;

        if y_top % y_bot != 0 {
            return 0;
        }
        
        let y = y_top / y_bot;

        let x_top = self.prize_x - self.button_bx * y;
        let x_bot = self.button_ax;

        if x_top % x_bot != 0 {
            return 0;
        }

        let x = x_top / x_bot;
        
        x * 3 + y
    }
}

impl FromStr for Claw {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.lines().collect();

        if parts.len() != 3 {
            return Err("Input must 3 lines".to_string());
        }

        let (buttona_left, buttona_right) = parts[0].split_once(", ").unwrap();
        let (_, buttonax) = buttona_left.split_once("+").unwrap();
        let (_, buttonay) = buttona_right.split_once("+").unwrap();

        let (buttonb_left, buttonb_right) = parts[1].split_once(", ").unwrap();
        let (_, buttonbx) = buttonb_left.split_once("+").unwrap();
        let (_, buttonby) = buttonb_right.split_once("+").unwrap();

        let (prize_left, prize_right) = parts[2].split_once(", ").unwrap();
        let (_, prizex) = prize_left.split_once("=").unwrap();
        let (_, prizey) = prize_right.split_once("=").unwrap();

        Ok(Claw {
            button_ax: buttonax.parse().unwrap(),
            button_ay: buttonay.parse().unwrap(),
            button_bx: buttonbx.parse().unwrap(),
            button_by: buttonby.parse().unwrap(),
            prize_x: prizex.parse::<isize>().unwrap(),
            prize_y: prizey.parse::<isize>().unwrap(),
        })
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|l| l.parse::<Claw>().unwrap().solve() as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|l| {
            let mut claw = l.parse::<Claw>().unwrap();
            claw.prize_x += 10000000000000;
            claw.prize_y += 10000000000000;
            claw.solve() as usize
        })
        .sum()
}
