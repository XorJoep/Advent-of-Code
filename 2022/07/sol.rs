use std::fs;
use std::time::Instant;

fn main() {
    let expect_result_part1 = 95437;
    let expect_result_part2 = 1;

    let filename_example = "ex_input";
    let filename = "input";

    let contents_example = fs
        ::read_to_string(filename_example)
        .expect("Erro reading the EXAMPLE file");
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

fn execute_part(part_fn: fn(&str) -> u32, input: &str, example_result: u32) -> bool {
    let start = Instant::now();
    let result = part_fn(&input);

    println!("\tFinished after {:?}", start.elapsed());
    println!("\tSolution found: {}", result);

    if example_result == 0 {
        println!("\tSkipping check");
        true
    } else if result != example_result {
        println!("\tINCORRECT: Expected: [{}] but got [{}]", example_result, result);
        false
    } else {
        println!("\tCORRECT");
        true
    }
}

#[derive(Debug)]
struct FileNode {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct DirNode {
    files: Vec<FileNode>,
    folders: Vec<Box<DirNode>>,
    name: String,
    parent: Option<Box<DirNode>>,
}

impl DirNode {
    pub fn new(name: &str, parent: Option<Box<DirNode>>) -> Self {
        DirNode {
            files: vec![],
            folders: vec![],
            name: name.to_string(),
            parent: parent,
        }
    }

    pub fn add_folder(&self, name: &str) {
        let new_node = DirNode::new(name, Some(Box::new(self)));
        self.folders.push(Box::new(new_node));
    }

    pub fn add_file(&self, name: &str, size: u64) {
        self.files.push(FileNode {
            name: name.to_string(),
            size: size
        })
    }
}

fn part1(input: &str) -> u32 {
    let dir = DirNode::new("/", None);
    println!("{:?}", dir);
    input
        .split("$ ")
        .skip(2) // skip line 1. We know the topdir is '/'
        .filter_map(|cmd| cmd.split_once("\n"))
        .for_each(|(instr, output)| {
            match instr {
                "ls" => {
                    output.lines().filter_map(|line| line.split_once(" ")).for_each(|(file, name)| {
                        match file {
                            "dir" => dir.add_folder(name),
                            _ => dir.add_file(name, file.parse::<u64>().expect("not an int")),
                        }
                    })
                },
                "cd .." => (),
                _ => (),
            };
            println!("{:?}: {:?}", instr, output)
        });

    1
}

fn part2(input: &str) -> u32 {
    input.lines().count() as u32
}