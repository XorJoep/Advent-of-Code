use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 0;
    let expect_result_part2 = 9021;

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
    let result = part_fn(input);

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
#[derive(Debug, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    Box,
    Fish,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Eq, PartialEq, Copy, Hash, Clone)]
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

    fn inverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn take_step(&self, position: &(isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.step();
        (position.0 as isize + dx, position.1 as isize + dy)
    }
}

use std::collections::{HashMap, HashSet};

fn ends_at_empty_spot(
    grid: &HashMap<(isize, isize), Tile>,
    start_pos: &(isize, isize),
    direction: &Direction,
) -> bool {
    let mut pos = direction.take_step(start_pos);
    while let Some(tile) = grid.get(&pos) {
        match tile {
            Tile::Empty => return true,
            Tile::Wall => return false,
            Tile::Box | Tile::Fish => {
                pos = direction.take_step(&pos);
            }
            _ => panic!("unknown tile"),
        }
    }
    false
}

fn get_all_affected_tiles(
    grid: &HashMap<(isize, isize), Tile>,
    start_pos: &(isize, isize),
    direction: &Direction,
) -> Vec<(isize, isize)> {
    let mut stack = vec![start_pos.clone()];
    let mut affected_tiles = Vec::new();
    let mut seen = HashSet::new();
    while let Some(pos) = stack.pop() {
        let new_pos = direction.take_step(&pos);
        if let Some(tile) = grid.get(&new_pos) {
            if seen.insert(new_pos) {
                match tile {
                    Tile::Empty => {
                        affected_tiles.push(new_pos);
                    }
                    Tile::Wall => {
                        return vec![];
                    }
                    Tile::BoxLeft => {
                        if matches!(direction, Direction::Down | Direction::Up) {
                            let extra_pos = Direction::Right.take_step(&new_pos);
                            stack.push(extra_pos);
                        }
                        affected_tiles.push(new_pos);
                        stack.push(new_pos);
                    }
                    Tile::BoxRight => {
                        if matches!(direction, Direction::Down | Direction::Up) {
                            let extra_pos = Direction::Left.take_step(&new_pos);
                            stack.push(extra_pos);
                        }
                        affected_tiles.push(new_pos);
                        stack.push(new_pos);
                    }
                    _ => panic!("unknown tile"),
                }
            }
        }
    }

    affected_tiles
}

fn part1(input: &str) -> usize {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
    let mut grid: HashMap<(isize, isize), Tile> = grid_input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => Tile::Fish,
                    _ => panic!("unknown tile"),
                };
                ((x as isize, y as isize), tile)
            })
        })
        .collect();

    let mut cur_pos = grid
        .iter()
        .find(|(_, v)| matches!(v, Tile::Fish))
        .map(|(&k, _)| k)
        .map(|(x, y)| (x, y))
        .unwrap();

    let moves = moves_input.chars().filter_map(|c| match c {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None,
    });

    for dir in moves {
        if !ends_at_empty_spot(&grid, &cur_pos, &dir) {
            continue;
        }

        let mut cur_tile = Tile::Fish;
        grid.insert(cur_pos, Tile::Empty);
        cur_pos = dir.take_step(&cur_pos);
        let mut walk_pos = cur_pos;

        while let Some(new_tile) = grid.get_mut(&walk_pos) {
            std::mem::swap(new_tile, &mut cur_tile);
            if matches!(cur_tile, Tile::Empty) {
                break;
            }
            walk_pos = dir.take_step(&walk_pos);
        }
    }

    grid.iter()
        .filter(|(_, t)| matches!(t, Tile::Box))
        .map(|((x, y), _)| x + y * 100)
        .sum::<isize>() as usize
}

fn part2(input: &str) -> usize {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
    let mut grid: HashMap<(isize, isize), Tile> = grid_input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().flat_map(move |(x, c)| {
                let (tile_left, tile_right) = match c {
                    '#' => (Tile::Wall, Tile::Wall),
                    '.' => (Tile::Empty, Tile::Empty),
                    'O' => (Tile::BoxLeft, Tile::BoxRight),
                    '@' => (Tile::Fish, Tile::Empty),
                    _ => panic!("unknown tile"),
                };
                let pos_left = (x as isize * 2, y as isize);
                let pos_right = (x as isize * 2 + 1, y as isize);
                vec![(pos_left, tile_left), (pos_right, tile_right)].into_iter()
            })
        })
        .collect();

    let mut cur_pos = grid
        .iter()
        .find(|(_, v)| matches!(v, Tile::Fish))
        .map(|(&k, _)| k)
        .unwrap();

    let moves = moves_input.chars().filter_map(|c| match c {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None,
    });

    for dir in moves {
        let mut affected_tiles = get_all_affected_tiles(&grid, &cur_pos, &dir);

        if affected_tiles.is_empty() {
            continue;
        }

        cur_pos = dir.take_step(&cur_pos);

        // Sort affected_tiles by y-coordinate depending on the direction
        affected_tiles.sort_by(|a, b| {
            if matches!(dir, Direction::Up) {
                b.1.cmp(&a.1)
            } else {
                a.1.cmp(&b.1)
            }
        });

        for pos in affected_tiles.iter().rev() {
            let rev_dir = dir.inverse();
            let to_swap_pos = rev_dir.take_step(pos);
            let tile1 = grid.get_mut(&pos).unwrap() as *mut Tile;
            let tile2 = grid.get_mut(&to_swap_pos).unwrap() as *mut Tile;
            // println!("swapping: {:?} <-> {:?}", pos, to_swap_pos);
            unsafe {
                // println!("swapping: {:?} <-> {:?}", *tile1, *tile2);
                std::ptr::swap(tile1, tile2);
            }
        }
    }

    // // Print out the tiles in the grid on their (x, y) location
    // for y in 0..=grid.keys().map(|(_, y)| *y).max().unwrap() {
    //     for x in 0..=grid.keys().map(|(x, _)| *x).max().unwrap() {
    //         if let Some(tile) = grid.get(&(x, y)) {
    //             match tile {
    //                 Tile::BoxLeft => print!("["),
    //                 Tile::BoxRight => print!("]"),
    //                 Tile::Fish => print!("@"),
    //                 Tile::Empty => print!("."),
    //                 Tile::Wall => print!("#"),
    //                 _ => panic!("unknown tile"),
    //             }
    //             // print!("{:?} ", tile);
    //         } else {
    //             print!("None ");
    //         }
    //     }
    //     println!();
    // }

    grid.iter()
        .filter(|(_, t)| matches!(t, Tile::BoxLeft))
        .map(|((x, y), _)| x + y * 100)
        .sum::<isize>() as usize
}
