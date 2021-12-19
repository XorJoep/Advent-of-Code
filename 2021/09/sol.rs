use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
        15;
    let expect_result_part2 = 
        1134;
    
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

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32) -> bool {
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

type Point<T> = (T, T);

pub struct Matrix {
	values: Vec<u32>,
	rows: usize,
	columns: usize
}

impl Matrix {
	pub fn new(input: &str) -> Matrix {
		Matrix {
            rows: input.lines().count(),
            columns: input.lines().nth(0).unwrap().chars().count(),
            values: input.lines().flat_map(|line| line.chars().map(|c| c.to_digit(10).expect("Not an int"))).collect::<Vec<u32>>()
        }
	}

    fn index(&self, r: usize, c: usize) -> usize {
		self.columns * r + c
	}

    fn get(&self, r: usize, c: usize) -> u32 {
		let index = self.index(r, c);
		self.values[index]
	}

    fn get_neighbours(&self, r: usize, c: usize) -> Vec<Point<usize>> {
        let mut ret = Vec::new();
        if r + 1 < self.rows {
            ret.push((r+1, c));
        }
        if r > 0 {
            ret.push((r-1, c));
        }
        if c + 1 < self.columns {
            ret.push((r, c+1));
        }
        if c > 0 {
            ret.push((r, c-1));
        }
        ret
    }

    fn get_basin(&self, r: usize, c: usize) -> Vec<Point<usize>> {
        let mut ret = Vec::new();
        let mut buff = vec![(r, c)];
        while !buff.is_empty() {
            let (x, y) = buff.pop().unwrap();
            ret.push((x, y));
            self.get_neighbours(x, y).iter().for_each(|(xn, yn)| {
                if !ret.contains(&(*xn, *yn)) && !buff.contains(&(*xn, *yn)) && self.get(*xn, *yn) != 9 {
                    buff.push((*xn, *yn))
                }
            })
        }

        ret
    }

    fn is_lowest(&self, r: usize, c: usize) -> Option<u32> {
        let val = self.get(r,c);
        if self.get_neighbours(r, c).iter().any(|(rr, cc)| self.get(*rr, *cc) <= val)
            {None}
        else {
            Some(val)
        }
    }
}

fn part1(input: &str) -> u32 {
    let matrix = Matrix::new(input);
    (0..matrix.rows)
        .map::<u32, _>(|r| {
            let low_vals = (0..matrix.columns)
                .filter_map(|c| {
                    matrix.is_lowest(r, c)
                })
                .collect::<Vec<u32>>();
            low_vals.iter().sum::<u32>() + low_vals.len() as u32
        })
    .sum::<u32>()
}


fn part2(input: &str) -> u32 {
    let matrix = Matrix::new(input);
    let mut basin_values = (0..matrix.rows)
        .flat_map(|r| {
            (0..matrix.columns)
                .filter(|c| {
                    matrix.is_lowest(r, *c).is_some()
                })
                .map(|c| matrix.get_basin(r, c).len() as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>();
    basin_values.sort();
    basin_values.iter().rev().take(3).product::<u32>()
}
