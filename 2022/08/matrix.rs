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
                        .map(|c| Tree { visible: false, height: c })
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
                print!("{}", self.get(x, y).height);
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

    pub fn get_rows(&mut self) -> Vec<&mut [Tree]> {
        self.values.chunks_mut(self.columns).collect()
    }

    // pub fn row_visibility(row: &[Tree]) -> usize {}

    pub fn calc_visibility(&mut self) -> u32 {
        self.values.chunks_mut(self.columns).for_each(|row| {
            let mut visibility = row
                .iter()
                .zip(row.iter().skip(1))
                .take_while(|&(t1, t2)| t1.height < t2.height)
                .count();

            row.iter_mut()
                .take(visibility + 1)
                .for_each(|t1| {
                    t1.visible = true;
                });

            visibility = row
                .iter()
                .rev()
                .zip(row.iter().rev().skip(1))
                .take_while(|&(t1, t2)| t1.height < t2.height)
                .count();

            row.iter_mut()
                .rev()
                .take(visibility + 1)
                .for_each(|t1| {
                    t1.visible = true;
                });
        });

        let column_indices: Vec<Vec<usize>> = (0..self.columns)
            .map(|c| (0..self.rows).map(|r| self.columns * r + c).collect())
            .collect();

        column_indices.iter().for_each(|column| {
            let mut visibility = column
                .iter()
                .rev()
                .zip(column.iter().rev().skip(1))
                .take_while(|&(t1, t2)| self.values[*t1].height < self.values[*t2].height)
                .count();

            column
                .iter()
                .rev()
                .take(visibility + 1)
                .for_each(|t1| {
                    self.values[*t1].visible = true;
                });

            visibility = column
                .iter()
                .zip(column.iter().rev().skip(1))
                .take_while(|&(t1, t2)| self.values[*t1].height < self.values[*t2].height)
                .count();

            column
                .iter()
                .take(visibility + 1)
                .for_each(|t1| {
                    self.values[*t1].visible = true;
                });
        });

        // println!("{:?}", column_indices);

        self.values
            .iter()
            .filter(|t| t.visible)
            .count() as u32
    }
}