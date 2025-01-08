use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 36;
    let expect_result_part2 = 81;

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn all_directions() -> &'static [Self] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn take_step(&self, position: (usize, usize)) -> (i32, i32) {
        let (dx, dy) = self.step();
        (position.0 as i32 + dx, position.1 as i32 + dy)
    }

    fn surrounding_locations(
        position: (usize, usize),
        x_bound: usize,
        y_bound: usize,
    ) -> Vec<(usize, usize)> {
        Self::all_directions()
            .iter()
            .map(|dir| dir.take_step(position))
            .filter_map(|new_pos| {
                // println!("{new_pos:?}");
                if new_pos.0 < 0
                    || new_pos.1 < 0
                    || new_pos.0 >= x_bound as i32
                    || new_pos.1 >= y_bound as i32
                {
                    None
                } else {
                    Some((new_pos.0 as usize, new_pos.1 as usize))
                }
            })
            .collect()
    }
}

fn part1(input: &str) -> u32 {
    let ysize = input.lines().count();
    let xsize = input.lines().next().unwrap_or("").chars().count();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let mut positions: HashMap<(usize, usize), HashSet<(usize, usize)>> = grid
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == 9 { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .map(|coord| (coord, HashSet::from([coord])))
        .collect();

    for i in (0..9).rev() {
        let mut new_positions: HashMap<(usize, usize), HashSet<(usize, usize)>>  = HashMap::new();
        positions.into_iter().for_each(|(k, v)| {
            Direction::surrounding_locations(k, xsize, ysize)
            .iter()
            .filter(|loc| grid[loc.1][loc.0] == i )
            .for_each(|&loc| {
                
                new_positions
                .entry(loc)
                        .and_modify(|existing| existing.extend(v.clone()))
                        .or_insert(v.clone());
                });
            });
            positions = new_positions.clone();
    }

    positions.iter().map(|(_, v)| v.len()).sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    let ysize = input.lines().count();
    let xsize = input.lines().next().unwrap_or("").chars().count();

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let mut positions: HashMap<(usize, usize), Vec<(usize, usize)>> = grid
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == 9 { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .map(|coord| (coord, Vec::from([coord])))
        .collect();

    for i in (0..9).rev() {
        let mut new_positions: HashMap<(usize, usize), Vec<(usize, usize)>>  = HashMap::new();
        positions.into_iter().for_each(|(k, v)| {
            Direction::surrounding_locations(k, xsize, ysize)
            .iter()
            .filter(|loc| grid[loc.1][loc.0] == i )
            .for_each(|&loc| {
                
                new_positions
                .entry(loc)
                        .and_modify(|existing| existing.extend(v.clone()))
                        .or_insert(v.clone());
                });
            });
            positions = new_positions.clone();
    }

    positions.iter().map(|(_, v)| v.len()).sum::<usize>() as u32
}
