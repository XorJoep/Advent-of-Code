use std::fs;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n")
    							 // .map(|s| s.parse().expect("parse error"))
    							 .collect();

  	let mut count = 0;
    for line in lines.iter() {
    	if line.is_empty() {
    		break;
    	}
    	let vals: Vec<&str> = line.split_whitespace().collect();
    	let (range, letter, pw) : (&str, char, &str) = (vals[0], vals[1].chars().nth(0).unwrap(), vals[2]);
    	let ranges: Vec<usize> = range.split("-").map(|s| s.parse().expect("parse error")).collect();
    	let (char_0, char_1) = (pw.chars().nth(ranges[0]-1), pw.chars().nth(ranges[1]-1));

        let mut found = 0;
        if char_0 != None && char_0.unwrap() == letter {
            found += 1;
        }
        if char_1 != None && char_1.unwrap() == letter{
            found += 1;
        }
    	
    	if found == 1 {
    		count += 1;
    		// println!("{}, {}, {}", range, num_found, count);
    	}
    }

    println!("{}", count);
  	
}