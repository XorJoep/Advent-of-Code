#![allow(dead_code)]
#![allow(unused_variables)]

type Point<T> = (T, T);

pub struct Tree {
    pub visible: bool,
    height: u32,
}

pub struct Matrix {
    values: Vec<Tree>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Matrix {
        let field_size = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count();
        Matrix {
            rows: field_size,
            columns: field_size,
            values: input
                .lines()
                .flat_map(|line|
                    line
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .map(|c| Tree { visible: false, height: c + 1 })
                )
                .collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn print(&self) {
        println!("------------");
        for y in 0..self.rows {
            for x in 0..self.columns {
                print!("{}", self.get(x, y).height - 1);
            }
            println!("");
        }
        println!("------------");
    }

    pub fn print_visible(&self) {
        println!("------------");
        for y in 0..self.rows {
            for x in 0..self.columns {
                print!("{}", if self.get(x, y).visible { "X" } else { "." });
            }
            println!("");
        }
        println!("------------");
    }

    fn get(&self, x: usize, y: usize) -> &Tree {
        let index = self.index(x, y);
        &self.values[index]
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.columns * y + x
    }

    pub fn activate(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.values[index].visible = true;
    }

    pub fn count_true(&self) -> u32 {
        self.values
            .iter()
            .filter(|v| v.visible)
            .count() as u32
    }

    pub fn get_rows(&self) -> Vec<Vec<u32>> {
        (0..self.columns)
            .map(|c| (0..self.rows).map(|r| self.values[self.columns * c + r].height).collect())
            .collect()
    }

    pub fn get_cols(&self) -> Vec<Vec<usize>> {
        (0..self.columns).map(|c| (0..self.rows).map(|r| self.columns * r + c).collect()).collect()
    }

    pub fn get_cols2(&self) -> Vec<Vec<u32>> {
        (0..self.columns)
            .map(|c| (0..self.rows).map(|r| self.values[self.columns * r + c].height).collect())
            .collect()
    }

    pub fn calc_visibility(&mut self) -> u32 {
        self.values.chunks_mut(self.columns).for_each(|row| {
            let mut highest = 0;
            row.iter_mut().for_each(|t| (
                if t.height > highest {
                    highest = t.height;
                    t.visible = true;
                }
            ));

            highest = 0;
            row.iter_mut()
                .rev()
                .for_each(|t| (
                    if t.height > highest {
                        highest = t.height;
                        t.visible = true;
                    }
                ));
        });

        self.get_cols()
            .iter()
            .for_each(|column| {
                let mut highest = 0;
                column.iter().for_each(|t| (
                    if self.values[*t].height > highest {
                        highest = self.values[*t].height;
                        self.values[*t].visible = true;
                    }
                ));
                highest = 0;
                column
                    .iter()
                    .rev()
                    .for_each(|t| (
                        if self.values[*t].height > highest {
                            highest = self.values[*t].height;
                            self.values[*t].visible = true;
                        }
                    ));
            });

        self.values
            .iter()
            .filter(|t| t.visible)
            .count() as u32
    }

    pub fn calc_2(&self) -> u32 {
        let rows = self.get_rows();
        let cols = self.get_cols2();

        (0..self.columns)
            .map(|c|
                (0..self.rows)
                    .map(|r| {
                        let tree_height = rows[r][c];
                        let mut biggest = 0;
                        let east = rows[r][c + 1..]
                            .iter()
                            .take_while(|&t| {
                                if tree_height <= biggest {
                                    false
                                } else if biggest <= *t {
                                    biggest = *t;
                                    true
                                } else if *t < biggest {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count();
                        biggest = 0;
                        let west = rows[r][..c]
                            .iter()
                            .rev()
                            .take_while(|&t| {
                                if tree_height <= biggest {
                                    false
                                } else if biggest <= *t {
                                    biggest = *t;
                                    true
                                } else if *t < biggest {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count();
                        biggest = 0;
                        let south = cols[c][r + 1..]
                            .iter()
                            .take_while(|&t| {
                                if tree_height <= biggest {
                                    false
                                } else if biggest <= *t {
                                    biggest = *t;
                                    true
                                } else if *t < biggest {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count();
                        biggest = 0;
                        let north = cols[c][..r]
                            .iter()
                            .rev()
                            .take_while(|&t| {
                                if tree_height <= biggest {
                                    false
                                } else if biggest <= *t {
                                    biggest = *t;
                                    true
                                } else if *t < biggest {
                                    true
                                } else {
                                    false
                                }
                            })
                            .count();

                        north * east * south * west;
                    })
                    .max()
                    .unwrap()
            )
            .max()
            .unwrap() as u32
    }
}