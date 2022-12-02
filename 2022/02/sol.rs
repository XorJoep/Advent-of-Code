use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let expect_result_part1 = 
        15;
    let expect_result_part2 = 
        12;
    
    let filename_example = "ex_input";
    let filename= "input";
    
    let contents_example = fs::read_to_string(filename_example).expect("Something went wrong reading the example file");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the example file");

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

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32) -> bool {
    let start = Instant::now();
    let result = part_fn(&input);

    println!("\tFinished after {:?}", start.elapsed());
    println!("\tSolution found: {}", result);

    let succes;
    if example_result == 0 {
        succes = true;
    }
    else if example_result != 0 && result != example_result {
        println!("\tINCORRECT: Expected: [{}] but got [{}]", example_result, result);
        succes = false
    }
    else {
        println!("\tExample CORRECT");
        succes = true;
    }

    succes
}

fn to_char(hand_char: &str) -> char {
    hand_char.chars().next().expect("no char found")
}

fn part1(input: &str) -> u32 {
    
    let rules = HashMap::from([
        (('A', 'X'), 1), // draw
        (('A', 'Y'), 0), // win
        (('A', 'Z'), 2), // lose
        (('B', 'X'), 2), // lose
        (('B', 'Y'), 1), // draw
        (('B', 'Z'), 0), // win
        (('C', 'X'), 0), // win
        (('C', 'Y'), 2), // lose
        (('C', 'Z'), 1)  // draw
    ]);

    let hand_value = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);

    let score = vec![ 
        6, // win
        3, // draw
        0  // lose
    ];

    input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(elf_hand, my_hand)| {
                let outcome = rules.get(&(to_char(elf_hand), to_char(my_hand)))
                    .expect("Rule not found");
                let hand_value = hand_value.get(&to_char(my_hand))
                    .expect("Hand not found");
                
                score[*outcome] + hand_value
            }
        )
        .sum()
}


fn part2(input: &str) -> u32 {
    let rules = HashMap::from([
        (('A', 'X'), 3), // lose + Z = 0 + 3 = 3
        (('A', 'Y'), 4), // draw + X = 3 + 1 = 4
        (('A', 'Z'), 8), // win + Y = 6 + 2 = 8
        (('B', 'X'), 1), // lose + X = 0 + 1 = 1
        (('B', 'Y'), 5), // draw + Y = 3 + 2 = 5
        (('B', 'Z'), 9), // win + Z = 6 + 3 = 9
        (('C', 'X'), 2), // lose + Y = 0 + 2 = 2
        (('C', 'Y'), 6), // draw + Z = 3 + 3 = 6
        (('C', 'Z'), 7)  // win + X = 6 + 1 = 7
    ]);

    input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .filter_map(|(elf_hand, my_hand)|
                rules.get(&(to_char(elf_hand), to_char(my_hand)))
        )
        .sum()
}
