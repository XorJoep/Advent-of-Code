use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 1930;
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

    fn find_all(position: (usize, usize), grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = HashSet::new();
        let mut visited = vec![vec![false; cols]; rows];

        Self::find_all_dfs(position, grid, &mut visited, &mut result);

        result
    }

    fn find_all_dfs(
        position: (usize, usize),
        grid: &Vec<Vec<char>>,
        seen: &mut Vec<Vec<bool>>,
        result: &mut HashSet<(usize, usize)>,
    ) {
        result.insert(position);
        seen[position.1][position.0] = true;

        for loc in Self::surrounding_locations(position, grid[0].len(), grid.len()) {
            if !seen[loc.1][loc.0] && grid[loc.1][loc.0] == grid[position.1][position.0] {
                Self::find_all_dfs(loc, grid, seen, result);
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut gardens: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    for x in 0..cols {
        for y in 0..rows {
            if seen.contains(&(x, y)) {
                continue;
            }

            let regions = Direction::find_all((x, y), &grid);
            seen.extend(regions.clone());
            gardens.insert((x, y), regions);
        }
    }

    let mut total = 0;

    for (k, v) in gardens {
        let mut count = 0;
        for loc in &v {
            count += Direction::all_directions()
                .iter()
                .map(|dir| dir.take_step(*loc))
                .filter(|new_pos| {
                    new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 >= cols as i32
                        || new_pos.1 >= rows as i32
                        || grid[new_pos.1 as usize][new_pos.0  as usize] != grid[k.1][k.0]
                })
                .count();
        }
        total += count * v.len();
    }

    total
}

fn part2(input: &str) -> usize {
    input.lines().count()
}
