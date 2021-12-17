use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
        5;
    let expect_result_part2 = 
        12;
    
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

type Point<T> = (T, T);
type Line<T> = (Point<T>, Point<T>);

pub struct Matrix {
	values: Vec<u32>,
	_rows: usize,
	columns: usize
}

impl Matrix {
	pub fn new(r: usize, c: usize) -> Matrix {
		Matrix {
            _rows: r,
            columns: c,
            values: vec![0; r*c]
        }
	}

    fn index(&self, r: usize, c: usize) -> usize {
		self.columns * r + c
	}

    fn incr(&mut self, r: usize, c: usize) {
		let index = self.index(r, c);
		self.values[index] += 1;
	}

    pub fn incr_line(&mut self, line: Line<u32>) {
        let ((mut r, mut c), (er, ec)) = line;
        loop {
            self.incr(r as usize, c as usize);
            if r == er && c == ec {break}
            if r < er { r += 1 }
            else if r > er { r -= 1}
            if c < ec { c += 1 }
            else if c > ec { c -= 1}
            
        }
	}
}

fn get_point(input: &str) -> Point<u32> {
    let (x_str, y_str) = input.split_once(",").unwrap();
    (x_str.parse().unwrap(), y_str.parse().unwrap())
}

fn is_diagonal(line: &Line<u32>) -> bool {
    let ((x1, y1), (x2, y2)) = line;
    x1 != x2 && y1 != y2
}

fn part1(input: &str) -> u32 {
    let mut matrix = Matrix::new(1000, 1000);
    input
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(p1, p2)| (get_point(p1), get_point(p2)))
        .filter(|line| !is_diagonal(line))
        .for_each(|line| matrix.incr_line(line));

    matrix.values.iter().filter(|&val| *val >= 2).count() as u32
}


fn part2(input: &str) -> u32 {
    let mut matrix = Matrix::new(1000, 1000);
    input
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(p1, p2)| (get_point(p1), get_point(p2)))
        .for_each(|line| matrix.incr_line(line));

    matrix.values.iter().filter(|&val| *val >= 2).count() as u32
}
