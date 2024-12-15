use std::fs;
use std::time::Instant;

use std::collections::HashSet;

fn main() {
    let expect_result_part1 = 41;
    let expect_result_part2 = 1;

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

    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn take_step(&self, position: (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.step();
        (position.0 + dx, position.1 + dy)
    }
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let x_size = grid[0].len() as i32;
    let y_size = grid.len() as i32;

    let start_location = grid
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            if let Some(x) = l.iter().position(|&c| c == '^') {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    let (mut pos_x, mut pos_y) = start_location;

    let mut current_direction = Direction::Up;

    let mut seen_positions = HashSet::new();
    seen_positions.insert(start_location);

    loop {
        let (new_x, new_y) = current_direction.take_step((pos_x, pos_y));
        if new_x < 0 || new_y < 0 || new_x >= x_size || new_y >= y_size {
            println!("Left the maze!");
            break;
        } else {
            match grid[new_y as usize][new_x as usize] {
                '.' | '^' => (pos_x, pos_y) = (new_x, new_y),
                '#' => current_direction = current_direction.turn(),
                _ => panic!("unknown char!"),
            }
            seen_positions.insert((pos_x, pos_y));
        }
    }
    seen_positions.len() as u32
}

fn part2(input: &str) -> u32 {
    input.lines().count() as u32
}
