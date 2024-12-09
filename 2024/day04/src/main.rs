use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 0;
    let expect_result_part2 = 9;

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

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32) -> bool {
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

fn find_xmas(haystack: &[char]) -> usize {
    let needle = vec!['X', 'M', 'A', 'S'];
    let rev_needle = vec!['S', 'A', 'M', 'X'];
    if haystack.len() < needle.len() {
        return 0;
    }

    haystack
        .windows(needle.len())
        .filter(|window| window == &needle || window == &rev_needle)
        .count()
}

fn part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_row = matrix.len();
    let max_col = matrix[0].len();

    let mut cols = vec![vec![]; max_col];
    let mut fdiag = vec![vec![]; max_row + max_col - 1];
    let mut bdiag = vec![vec![]; fdiag.len()];

    for x in 0..max_col {
        for y in 0..max_row {
            cols[x].push(matrix[y][x]);
            fdiag[x + y].push(matrix[y][x]);
            let diag_min = (x + max_row) as i32 - (y + 1) as i32;
            if diag_min < 0 {
                panic!("{diag_min} < 0");
            }
            bdiag[diag_min as usize].push(matrix[y][x]);
        }
    }

    let mut xmasses = 0;
    for l in matrix {
        let xmas_found = find_xmas(&l);
        xmasses += xmas_found;
    }

    for l in cols {
        let xmas_found = find_xmas(&l);
        xmasses += xmas_found;
    }

    for l in fdiag {
        let xmas_found = find_xmas(&l);
        xmasses += xmas_found;
    }

    for l in bdiag {
        let xmas_found = find_xmas(&l);
        xmasses += xmas_found;
    }

    xmasses as u32
}

fn part2(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_row = matrix.len();
    let max_col = matrix[0].len();

    let mut count = 0;

    for x in 1..max_col - 1 {
        for y in 1..max_row - 1 {
            if matrix[y][x] != 'A' {
                continue;
            }

            let topleft = matrix[y - 1][x - 1];
            let topright = matrix[y - 1][x + 1];
            let botleft = matrix[y + 1][x - 1];
            let botright = matrix[y + 1][x + 1];

            if topleft == botright || topright == botleft {
                continue;
            }

            let vals = vec![topleft, topright, botleft, botright];
            let ms = vals.iter().filter(|v| **v == 'M').count();
            let ss = vals.iter().filter(|v| **v == 'S').count();

            println!("{x}, {y}");

            if ms == 2 && ss == 2 {
                count += 1;
            }
        }
    }

    count
}
