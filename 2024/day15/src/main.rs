use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 10092;
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
#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Empty,
    Box,
    Fish,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn take_step(&self, position: &(isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.step();
        (position.0 as isize + dx, position.1 as isize + dy)
    }
}

use std::collections::HashMap;

fn ends_at_empty_spot(
    grid: &HashMap<(isize, isize), Tile>,
    start_pos: &(isize, isize),
    direction: &Direction,
) -> bool {
    let mut pos = *start_pos;
    loop {
        pos = direction.take_step(&pos);
        match grid[&pos] {
            Tile::Empty => return true,
            Tile::Wall => return false,
            Tile::Box => continue,
            _ => panic!("help"),
        }
    }
}

fn part1(input: &str) -> usize {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
    let mut grid: HashMap<(isize, isize), Tile> = grid_input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                (
                    (x as isize, y as isize),
                    match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'O' => Tile::Box,
                        '@' => Tile::Fish,
                        _ => panic!("unknown tile"),
                    },
                )
            })
        })
        .flatten()
        .collect();

    let mut cur_pos = grid
        .iter()
        .find(|(_, v)| matches!(v, Tile::Fish))
        .map(|(&k, _)| k)
        .map(|(x, y)| (x, y))
        .unwrap();

    let moves: Vec<Direction> = moves_input
        .lines()
        .map(|l| {
            l.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("unknown tile"),
            })
        })
        .flatten()
        .collect();

    for dir in moves {
        if !ends_at_empty_spot(&grid, &cur_pos, &dir) {
            continue;
        }

        let mut cur_tile = Tile::Fish;
        grid.insert(cur_pos, Tile::Empty);
        cur_pos = dir.take_step(&cur_pos);
        let mut walk_pos = cur_pos;
        loop {
            let new_tile = grid[&walk_pos].clone();
            grid.insert(walk_pos, cur_tile);
            cur_tile = new_tile.clone();
            if matches!(cur_tile, Tile::Empty) {
                break;
            }
            walk_pos = dir.take_step(&walk_pos);
        }
    }

    println!("{grid:?}");

    grid.iter()
        .filter(|(_, t)| matches!(t, Tile::Box))
        .map(|((x, y), _)| x + y * 100)
        .sum::<isize>() as usize
}

fn part2(input: &str) -> usize {
    input.lines().count()
}
