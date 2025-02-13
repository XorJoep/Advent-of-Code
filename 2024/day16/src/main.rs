use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    position: (isize, isize),
    cost: usize,
    direction: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn step(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

fn flood_fill(
    grid: &HashMap<(isize, isize), Tile>,
    start: (isize, isize),
    end: (isize, isize),
) -> Option<usize> {
    let mut open_set = BinaryHeap::new();
    let mut g_score = HashMap::new();
    let mut came_from = HashMap::new();

    open_set.push(Node {
        position: start,
        cost: 0,
        direction: Direction::Right,
    });

    g_score.insert(start, 0);

    while let Some(Node {
        position,
        cost,
        direction,
    }) = open_set.pop()
    {
        // if position == end {
        //     return Some(cost);
        // }

        for &dir in &Direction::all() {
            let neighbor = dir.step(position);
            if let Some(Tile::Wall) = grid.get(&neighbor) {
                continue;
            }

            let turn_cost = if direction != dir { 1000 } else { 0 };

            let tentative_g_score = cost + 1 + turn_cost;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, position);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    direction: dir,
                });
            }
        }
    }

    println!("g_scores: {:?}", g_score);

    g_score.get(&end).copied()
}

fn main() {
    let expect_result_part1 = 1;
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

fn part1(input: &str) -> usize {
    let floor: HashMap<(isize, isize), Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let pos = (x as isize, y as isize);
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => panic!("Invalid character"),
                };
                (pos, tile)
            })
        })
        .collect();

    let start = floor
        .iter()
        .find_map(
            |(&pos, &tile)| {
                if tile == Tile::Start {
                    Some(pos)
                } else {
                    None
                }
            },
        )
        .expect("No start found");

    let end = floor
        .iter()
        .find_map(
            |(&pos, &tile)| {
                if tile == Tile::End {
                    Some(pos)
                } else {
                    None
                }
            },
        )
        .expect("No end found");

    println!("start: {:?}, end: {:?}", start, end);

    match flood_fill(&floor, start, end) {
        Some(cost) => cost,
        None => panic!("No path found"),
    }
}

fn part2(input: &str) -> usize {
    input.lines().count()
}
