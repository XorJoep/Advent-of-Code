use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let expect_result_part1 = 0;
    let expect_result_part2 = 0;

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
    // println!("Part 2 - Example");
    // succes = succes && execute_part(part2, &contents_example, expect_result_part2);
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
#[derive(Debug)]
pub struct Robot {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn advance(&mut self, grid_x: isize, grid_y: isize, seconds: isize) {
        self.px = modulo(self.px + self.vx * seconds, grid_x);
        self.py = modulo(self.py + self.vy * seconds,grid_y);
    }
}

#[derive(Debug)]
pub struct Grid {
    xsize: isize,
    ysize: isize,
    robots: Vec<Robot>,
}

impl Grid {
    fn advance(&mut self, seconds: isize) {
        for robot in &mut self.robots {
            robot.advance(self.xsize, self.ysize, seconds);
        }
    }

    fn quadrant_sum(&self) -> usize {
        let x_mid = self.xsize / 2;
        let y_mid = self.ysize / 2;

        let mut quadrants = (0,0,0,0);
        for robot in &self.robots {
            if robot.px < x_mid && robot.py < y_mid {
                quadrants.0 += 1;
            } else if robot.px > x_mid && robot.py < y_mid {
                quadrants.1 += 1;
            } else if robot.px > x_mid && robot.py > y_mid {
                quadrants.2 += 1;
            } else if robot.px < x_mid && robot.py > y_mid {
                quadrants.3 += 1;
            }
        }

        // println!("{quadrants:?}");

        quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3 
    }

    fn print(&self) {
        let robot_set: HashSet<(isize, isize)> = self.robots.iter().map(|r| (r.px, r.py)).collect();
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                if robot_set.contains(&(x,y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

fn modulo(a: isize, n: isize) -> isize {
    ((a % n) + n) % n
}

fn part1(input: &str) -> usize {
    let x_size = 101;
    let y_size = 103;

    let robots: Vec<Robot> = input
        .lines()
        .map(|l| {
            let (p, v) = l[2..].split_once(" v=").unwrap();
            let (px, py) = p.split_once(",").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();

            Robot {
                px: px.parse().unwrap(),
                py: py.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect();

    let mut grid = Grid {xsize: x_size, ysize: y_size, robots: robots};

    grid.advance(100);



    // println!("{grid:?}");

    grid.quadrant_sum()
}

use std::thread;
use std::time::Duration;

use std::io::Write;
fn part2(input: &str) -> usize {
    let x_size = 101;
    let y_size = 103;

    let robots: Vec<Robot> = input
        .lines()
        .map(|l| {
            let (p, v) = l[2..].split_once(" v=").unwrap();
            let (px, py) = p.split_once(",").unwrap();
            let (vx, vy) = v.split_once(",").unwrap();

            Robot {
                px: px.parse().unwrap(),
                py: py.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect();

    let mut grid = Grid {xsize: x_size, ysize: y_size, robots: robots};

    grid.advance(7344);
    grid.print();

    // thread::sleep(Duration::from_millis(1000));
    
    // for n in 0..100 {
    //     print!("\x1B[2J\x1B[H");
    //     std::io::stdout().flush().unwrap();
        
    //     grid.advance(103);

    //     grid.print();

    //     println!("NUMBER: {}", n);

    //     thread::sleep(Duration::from_millis(1000));
    // }

    0
}
