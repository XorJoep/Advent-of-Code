use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn main() {
    let expect_result_part1 = 14;
    let expect_result_part2 = 34;

    let filename_example = "ex_input";
    let filename = "input";

    let contents_example = fs
        ::read_to_string(filename_example)
        .expect("Erro reading the EXAMPLE file");
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
        println!("\tINCORRECT: Expected: [{}] but got [{}]", example_result, result);
        false
    } else {
        println!("\tCORRECT");
        true
    }
}

fn part1(input: &str) -> u32 {
    let mut antennas = HashMap::new();

    let ysize = input.lines().count() as i32;
    let xsize = input.lines().next().unwrap_or("").chars().count() as i32;

    input.lines().enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push((x as i32,y as i32));
            }
        })
    });

    let mut antinodes = HashSet::new();

    for (_antenna, locs) in &antennas {
        for i in 0..locs.len() {
            for j in i+1..locs.len() {
                let vector = (locs[i as usize].0 - locs[j as usize].0, locs[i as usize].1 - locs[j as usize].1);
                // println!("{locs:?} -> {i}, {j} -> {vector:?}");

                let antinode_1 = (locs[i].0 + vector.0, locs[i].1 + vector.1);
                
                if antinode_1.0 >= 0 && antinode_1.1 >= 0 && antinode_1.0 < xsize && antinode_1.1 < ysize {
                    antinodes.insert(antinode_1);
                }
                let antinode_2 = (locs[j].0 - vector.0, locs[j].1 - vector.1);
                if antinode_2.0 >= 0 && antinode_2.1 >= 0 && antinode_2.0 < xsize && antinode_2.1 < ysize {
                    antinodes.insert(antinode_2);
                }
            }
        }
    }

    antinodes.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut antennas = HashMap::new();

    let ysize = input.lines().count() as i32;
    let xsize = input.lines().next().unwrap_or("").chars().count() as i32;

    input.lines().enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push((x as i32,y as i32));
            }
        })
    });

    let mut antinodes = HashSet::new();

    for (_antenna, locs) in &antennas {
        for i in 0..locs.len() {
            antinodes.insert(locs[i]);
            for j in i+1..locs.len() {
                let vector = (locs[i as usize].0 - locs[j as usize].0, locs[i as usize].1 - locs[j as usize].1);
                // println!("{locs:?} -> {i}, {j} -> {vector:?}");

                let mut antinode_1 = (locs[i].0 + vector.0, locs[i].1 + vector.1);
                while antinode_1.0 >= 0 && antinode_1.1 >= 0 && antinode_1.0 < xsize && antinode_1.1 < ysize {
                    println!("1: {antinode_1:?}, {vector:?}");
                    antinodes.insert(antinode_1);
                    antinode_1 = (antinode_1.0 + vector.0, antinode_1.1 + vector.1);
                }

                let mut antinode_2 = (locs[i].0 -vector.0, locs[i].1 -vector.1);
                while antinode_2.0 >= 0 && antinode_2.1 >= 0 && antinode_2.0 < xsize && antinode_2.1 < ysize {
                    println!("2: {antinode_2:?}, {vector:?}");
                    antinodes.insert(antinode_2);
                    antinode_2 = (antinode_2.0 - vector.0, antinode_2.1 - vector.1);
                }
            }
        }
    }

    for y in 0..ysize {
        for x in 0..xsize {
            if antinodes.contains(&(x,y)) {
                print!("#");
            }
            else {
                print!(".");
            }
            
        }
        println!("");
    }

    antinodes.len() as u32
}