use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 
    4512;
    let expect_result_part2 = 
    1924;
    
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

// ----------------------------------------------------------------------------------------

type BingoCard = Vec<Option<u32>>;
type BingoCards = Vec<Option<BingoCard>>;

fn new_bingo<'a>(cards: &'a mut BingoCards, num: &u32) -> Option<&'a mut BingoCard> {
    let mut res_card = None;
    for card in cards.iter_mut() {
        if let Some(ref mut cardref) = card {
            if check_bingo(cardref) {continue}
            for val in cardref.iter_mut() {
                if *val == Some(*num) {
                    *val = None;
                    if check_bingo(cardref) {
                        res_card = Some(cardref);
                    }
                    break
                }
            }
        }
    }

    return res_card;
}

fn check_bingo(card: &BingoCard) -> bool {
    (0..5).any(|h|
            ((0..5)
                .all(|v| 
                    card[h+v*5].is_none()))
            ||
            ((0..5)
                .all(|v| 
                    card[h*5+v].is_none()))
        )
}

fn part1(input: &str) -> u32 {
    let mut cards: BingoCards = 
    input
        .split("\r\n\r\n")
        .skip(1)
        .map(|card| 
            Some(card
                .split_ascii_whitespace()
                .map(|s| Some(s.parse::<u32>().expect("parse error")))
                .collect()
            )
        ).collect();
        
    let num = 
    input
        .split("\r\n\r\n")
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().expect("parse error"))
        .find_map(|num| {
            let card = new_bingo(&mut cards, &num);
            if card.is_some() {
                Some((card.unwrap().iter().fold(0, |acc, v| acc + v.unwrap_or(0))) * num)
            }
            else {
                None
            }
        })
        .unwrap();

    num
}


fn part2(input: &str) -> u32 {
        let mut cards: BingoCards = 
    input
        .split("\r\n\r\n")
        .skip(1)
        .map(|card| 
            Some(card
                .split_ascii_whitespace()
                .map(|s| Some(s.parse::<u32>().expect("parse error")))
                .collect()
            )
        ).collect();
        
    let num = 
    input
        .split("\r\n\r\n")
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().expect("parse error"))
        .map(|num| {
            let card = new_bingo(&mut cards, &num);
            if card.is_some() {
                Some((card.unwrap().iter().fold(0, |acc, v| acc + v.unwrap_or(0))) * num)
            }
            else {
                None
            }
        })
        .filter(|x| x.is_some())
        .last()
        .unwrap()
        .unwrap();

    num
}
