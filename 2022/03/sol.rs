use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
        157;
    let expect_result_part2 = 
        70;
    
    let filename_example = "ex_input";
    let filename = "input";
    
    let contents_example = fs::read_to_string(filename_example).expect("Erro reading the EXAMPLE file");
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


fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (c1, c2) = 
                line.split_at(
                        line.chars().count() / 2
                    );

             c1.chars()
                .filter(|c| c2.contains(*c))
                .next()
        })
        .map(|c| c as u32) // char to integer
        .map(|c|
                if c > 0x60 { c - 0x60 }
                else { c - 0x40 + 26 } 
        )
        .sum()
}


fn part2(input: &str) -> u32 {
    let bags: Vec<&str> = input.lines().collect();
    bags
        .chunks(3)
        .filter_map(|group| 
            group[0]
                .chars()
                .filter(|c| group[1].contains(*c))
                .filter(|c| group[2].contains(*c))
                .next()
        )
        .map(|c| c as u32)
        .map(|c|
                if c > 0x60 { c - 0x60 }
                else { c - 0x40 + 26 } 
        )
        .sum()
}
