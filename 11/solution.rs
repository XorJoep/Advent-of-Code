use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mapping = parse_input(&contents);

    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&mapping));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&mapping));
    println!("Finished after {:?}", start.elapsed());
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes()
                  .collect::<Vec<u8>>()
            )
        .collect::<Vec<Vec<u8>>>()
}

fn look_around(mapping: &Vec<Vec<u8>>, row: usize, col: usize) -> (usize, usize) {
    let mut empty = 0;
    let mut occupied = 0;
    let row_length = mapping.len() as i32;
    let col_length = mapping[0].len() as i32;
    let i_row = row as i32;
    let i_col = col as i32;
    for i in -1..2 {
        if i_row + i >= row_length || i_row + i < 0 {continue}
        for j in -1..2 {
            if i_col + j >= col_length || i_col + j < 0 {continue}
            else if i == 0 && j == 0 {continue}
            match mapping[(i_row+i) as usize][(i_col+j) as usize] {
                b'#' => occupied += 1,
                b'L' => empty += 1,
                _ => {},
            }
        }
    }
    (empty, occupied)
}

fn look_around_2(mapping: &Vec<Vec<u8>>, row: usize, col: usize) -> (usize, usize) {
    let mut empty = 0;
    let mut occupied = 0;
    let row_length = mapping.len() as i32;
    let col_length = mapping[0].len() as i32;
    let i_row = row as i32;
    let i_col = col as i32;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {continue}
            for step in 1.. {
                if i_row + i*step >= row_length || i_row + i*step < 0 {break}
                else if i_col + j*step >= col_length || i_col + j*step < 0 {break}
                else {
                    match mapping[(i_row+i*step) as usize][(i_col+j*step) as usize] {
                        b'#' => {occupied += 1; break;},
                        b'L' => {empty += 1; break;},
                        _ => {},
                    }
                }
            }
        }
    }
    (empty, occupied)
}


fn occupied(mapping: &Vec<Vec<u8>>) -> usize{
    mapping.iter().map(|row| row.iter().filter(|&seat| *seat == b'#').count()).sum()
}

fn advance_state(mapping: &mut Vec<Vec<u8>>, 
                 look_around_fn: fn(&Vec<Vec<u8>>, usize, usize) -> (usize, usize), 
                 occupied_red: usize) -> bool {
    let mut map = mapping.clone();
    let mut change = false;
    for row in 0..mapping.len() {
        for col in 0..mapping[row].len() {
            let val = mapping[row][col];
            if val == b'.' {continue}
            let (_empty, occupied) = look_around_fn(mapping, row, col);
            match val {
                b'#' => {
                    if occupied >= occupied_red {map[row][col] = b'L'; change = true;}
                },
                b'L' => {
                    if occupied == 0 {map[row][col] = b'#'; change = true;}
                },
                _ => {}
            }
        }
    }
    *mapping = map;
    change
}

fn part1(mapping: &Vec<Vec<u8>>) -> usize {
    let mut mut_map = mapping.clone();
    loop {
        if !advance_state(&mut mut_map, look_around, 4) {
            return occupied(&mut_map);
        }
    }
}

fn part2(mapping: &Vec<Vec<u8>>) -> usize {
    let mut mut_map = mapping.clone();
    loop {
        if !advance_state(&mut mut_map, look_around_2, 5) {
            return occupied(&mut_map);
        }
    }
}