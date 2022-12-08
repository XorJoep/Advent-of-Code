use std::fs;
use std::time::Instant;
use std::collections::HashMap;

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
    id: usize,
    size: u32,
    files: Option<Vec<usize>>,
    parent: Option<usize>,
}

impl FileNode {
    pub fn new_file(id: usize, parent: Option<usize>, size: u32) -> Self {
        FileNode {
            id: id,
            parent: parent,
            size: size,
            files: None,
        }
    }

    pub fn new_folder(id: usize, parent: Option<usize>) -> Self {
        FileNode {
            id: id,
            parent: parent,
            size: 0,
            files: Some(vec![]),
        }
    }

    pub fn get_size(&self, stack: &Vec<FileNode>) -> u32 {
        match self.files {
            Some(_) => {
                self.files.as_ref().unwrap()
                    .iter()
                    .map(|file_id| stack[*file_id].get_size(stack))
                    .sum()
            }
            None => self.size
        }
    }

    pub fn small_folder_size(&self, stack: &Vec<FileNode>) -> u32 {
        match self.files {
            Some(_) => {
                let size = self.get_size(stack);
                if size < 100000 {
                    size
                }
                else  { 0 }
            }
            None => 0
        }
    }

    // pub fn add_folder(stack: &mut Vec<FileNode>, name: &str, parent: Option<usize>) {}
}

fn part1(input: &str) -> u32 {
    let mut stack: Vec<FileNode> = vec![FileNode::new_folder(0, None)];
    let mut dir_map = HashMap::new();
    let mut cur_dir = 0;

    // println!("{:?}", stack);
    input
        .split("$ ")
        .skip(2) // skip line 1. We know the topdir is '/'
        .filter_map(|cmd| cmd.split_once("\n"))
        .for_each(|(instr, output)| {
            match instr {
                "ls" => {
                    output
                        .lines()
                        .filter_map(|line| line.split_once(" "))
                        .for_each(|(file, name)| {
                            let new_id = stack.len();
                            match file {
                                "dir" => {
                                    stack[cur_dir].files
                                        .as_mut()
                                        .expect("cannot push to file")
                                        .push(new_id);
                                    stack.push(FileNode::new_folder(new_id, Some(cur_dir)));
                                    dir_map.insert(name, new_id);
                                }
                                _ => {
                                    stack[cur_dir].files
                                        .as_mut()
                                        .expect("cannot push to file")
                                        .push(new_id);
                                    stack.push(
                                        FileNode::new_file(
                                            new_id,
                                            Some(stack[cur_dir].id),
                                            file.parse::<u32>().expect("not an int")
                                        )
                                    );
                                }
                            }
                        });
                }
                "cd .." => {
                    cur_dir = stack[cur_dir].parent.unwrap();
                }
                _ => {
                    // cd xxx
                    let (_, dirname) = instr.split_once(" ").unwrap();
                    cur_dir = *dir_map.get(dirname).expect("could not find dir");
                }
            }
        });

    println!("{:?}", stack);

    // stack[0]

    // match self.files {
    //     Some(_) => self.size = self.files.iter().map(|file| file.calc_size()).sum(),
    //     None => ()
    // }
    // self.size

    stack
        .iter()
        .filter(|node| node.size == 0)
        // .filter_map(|folder| folder.files.as_ref())
        .map(|folder|
            folder.small_folder_size(&stack)
        ).sum::<u32>() as u32
}

fn part2(input: &str) -> u32 {
    input.lines().count() as u32
}