use std::fs;
use std::time::Instant;

fn main() {
    let filename = "input";
    //let filename = "ex_input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let start = Instant::now();
    println!("Solution for PART 1: {}", part1(&contents));
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    println!("Solution for PART 2: {}", part2(&contents));
    println!("Finished after {:?}", start.elapsed());
}



fn part1(input: &str) -> u32 {
    let hsize: usize = 12;
    let linecount =     
    input
        .lines()
        .count();

    let gamma_string: String = 
    input
        .lines()
        .fold(vec![0; hsize], |collection, line| {
            collection.iter()
                .zip(line.chars())
                .map(|(count, val)| 
                    count + val.to_digit(10).unwrap()
                ).collect::<Vec<u32>>()
        })
        .iter()
        .map(|&v| char::from_u32((v > (linecount / 2) as u32) as u32 + '0' as u32).unwrap())
        .collect();
    let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();

    gamma * (!gamma & ((1 << hsize) - 1))
}

fn check_more_ones(input: &Vec<&str>, hoffset: usize, vsize: usize) -> bool {
    input
        .iter()
        .filter(|binstr|
            binstr.chars().nth(hoffset).unwrap() == '0'
        )
        .count() <= vsize
}

fn get_oxygen_generator_rating(mut bins: Vec<&str>, mut vsize: usize) -> u32 {
    let mut hoffset = 0;
    while vsize > 1 {
        let more_ones = check_more_ones(&bins, hoffset, vsize/2 as usize);
        bins = bins.iter().filter(|bin| ! ((bin.chars().nth(hoffset).unwrap() == '1') ^ more_ones)).cloned().collect();
        vsize = bins.iter().count();
        hoffset += 1;
    }

    u32::from_str_radix(bins[0], 2).unwrap()
}

fn get_co2_scrubbed_rating(mut bins: Vec<&str>, mut vsize: usize) -> u32 {
    let mut hoffset = 0;
    while vsize > 1 {
        let more_ones = !check_more_ones(&bins, hoffset, vsize/2 as usize);
        bins = bins.iter().filter(|bin| ! ((bin.chars().nth(hoffset).unwrap() == '1') ^ more_ones)).cloned().collect();
        vsize = bins.iter().count();
        hoffset += 1;
    }

    u32::from_str_radix(bins[0], 2).unwrap()
}

fn part2(input: &str) -> u32 {
    let bins: Vec<&str> = 
    input
        .lines()
        .collect();
    let vsize: usize = bins.iter().count();

    get_oxygen_generator_rating(bins.clone(), vsize) * get_co2_scrubbed_rating(bins.clone(), vsize)

}
