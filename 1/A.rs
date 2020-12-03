use std::fs;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let nums: Vec<i32> = contents.split_whitespace()
    							 .map(|s| s.parse().expect("parse error"))
    							 .collect();

    for (i, k) in nums.iter().enumerate() {
    	let r = nums[i..].iter().map(|n| k+n).position(|k| k==2020);
    	if r != None {
    		println!("Found sum at {0} {1} with multiple {2}", 
    			nums[i], nums[i + r.unwrap()], nums[i] * nums[i + r.unwrap()]);
    	}
    }
}