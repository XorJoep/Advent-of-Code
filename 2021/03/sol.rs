use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
        1;
    let expect_result_part2 = 
        1;
    
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

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32,) -> bool {
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
    let hsize: usize = 12;
    let linecount =     
    input
        .lines()
        .count();

    let gamma_string: String = 
    input
        .lines()
        .fold(vec![0; hsize], |collection, line| {
            collection.iter()
                .zip(line.chars())
                .map(|(count, val)| 
                    count + val.to_digit(10).unwrap()
                ).collect::<Vec<u32>>()
        })
        .iter()
        .map(|&v| char::from_u32((v > (linecount / 2) as u32) as u32 + '0' as u32).unwrap())
        .collect();
    let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();

    gamma * (!gamma & ((1 << hsize) - 1))
}

fn check_more_ones(input: &Vec<&str>, hoffset: usize, vsize: usize) -> bool {
    input
        .iter()
        .filter(|binstr|
            binstr.chars().nth(hoffset).unwrap() == '0'
        )
        .count() <= vsize
}

fn get_oxygen_generator_rating(mut bins: Vec<&str>, mut vsize: usize) -> u32 {
    let mut hoffset = 0;
    while vsize > 1 {
        let more_ones = check_more_ones(&bins, hoffset, vsize/2 as usize);
        bins = bins.iter().filter(|bin| ! ((bin.chars().nth(hoffset).unwrap() == '1') ^ more_ones)).cloned().collect();
        vsize = bins.iter().count();
        hoffset += 1;
    }

    u32::from_str_radix(bins[0], 2).unwrap()
}

fn get_co2_scrubbed_rating(mut bins: Vec<&str>, mut vsize: usize) -> u32 {
    let mut hoffset = 0;
    while vsize > 1 {
        let more_ones = !check_more_ones(&bins, hoffset, vsize/2 as usize);
        bins = bins.iter().filter(|bin| ! ((bin.chars().nth(hoffset).unwrap() == '1') ^ more_ones)).cloned().collect();
        vsize = bins.iter().count();
        hoffset += 1;
    }

    u32::from_str_radix(bins[0], 2).unwrap()
}

fn part2(input: &str) -> u32 {
    let bins: Vec<&str> = 
    input
        .lines()
        .collect();
    let vsize: usize = bins.iter().count();

    get_oxygen_generator_rating(bins.clone(), vsize) * get_co2_scrubbed_rating(bins.clone(), vsize)

}
