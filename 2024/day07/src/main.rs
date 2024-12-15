use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 0;
    let expect_result_part2 = 11387;
    // let expect_result_part2 = 11387;

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

fn execute_part(part_fn: fn(&str) -> u64, input: &str, example_result: u64) -> bool {
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

fn calc(total: u64, to_calc: &Vec<u64>, index: usize, to_find: u64, p2: bool) -> bool {
    if total == to_find && index == to_calc.len() {
        return true;
    }

    if index >= to_calc.len() {
        return false;
    }

    if p2 {
        let log10 = to_calc[index].ilog10() + 1;
        let concat = total * 10_u64.pow(log10) + to_calc[index];

        if calc(total + to_calc[index], to_calc, index + 1, to_find, p2) {
            print!(" {} +", to_calc[index]);
            true
        } else if calc(total * to_calc[index], to_calc, index + 1, to_find, p2){
            print!(" {} *", to_calc[index]);
            true
        } else if calc(concat, to_calc, index + 1, to_find, p2){
            print!(" {} ||", to_calc[index]);
            true
        } else {
            false
        }
    } else {
        calc(total + to_calc[index], to_calc, index + 1, to_find, p2)
            || calc(total * to_calc[index], to_calc, index + 1, to_find, p2)
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(solve, nums)| {
            let numvec: Vec<u64> = nums.split(" ").map(|i| i.parse().unwrap()).collect();
            let to_solve: u64 = solve.parse().unwrap();

            let solvable = calc(numvec[0], &numvec, 1, to_solve, false);
            if solvable {
                Some(to_solve)
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(solve, nums)| {
            let numvec: Vec<u64> = nums.split(" ").map(|i| i.parse().unwrap()).collect();
            let to_solve: u64 = solve.parse().unwrap();

            println!("{to_solve} -> {numvec:?}");
            let solvable = calc(numvec[0], &numvec, 1, to_solve, true);
            print!(" {}", numvec[0]);
            println!(" ");
            println!("-> {solvable}");

            if solvable {
                Some(to_solve)
            } else {
                None
            }
        })
        .sum::<u64>()
}
