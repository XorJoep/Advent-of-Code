#![allow(dead_code)]
#![allow(unused_variables)]


type Point<T> = (T, T);


pub struct Matrix {
	values: Vec<bool>,
	rows: usize,
	columns: usize,
    folded_rows: usize,
	folded_columns: usize
}

impl Matrix {
	pub fn new(x: usize, y: usize) -> Matrix {
		Matrix {
            rows: y,
            columns: x,
            folded_rows: y,
            folded_columns: x,
            values: vec![false; x*y],
        }
	}

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn print(&self) {
        println!("------------");
        for y in 0..self.folded_rows {
            for x in 0..self.folded_columns {
                print!("{}", if self.get(x,y) {"#"} else {"."});
            }
            println!("");
        }
        println!("------------");
    }

    pub fn fold(&mut self, i: usize, horizontal: bool) {
        if horizontal {
            for y in 0..self.folded_rows {
                for x in 0..i {
                    self.fold_single(x,y,self.folded_columns-x-1, y);
                }
            }
            self.folded_columns = i;
        } else {
            for x in 0..self.folded_columns{
                for y in 0..i-1 {
                    self.fold_single(x, y, x, self.folded_rows-y-1);
                }
            }
            self.folded_rows = i;
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
		let index = self.index(x,y);
        self.values[index]
	}

    fn index(&self, x: usize, y: usize) -> usize {
		self.columns * y + x
	}

    fn fold_single(&mut self, x: usize, y: usize, x2: usize, y2: usize) {
        let index = self.index(x, y);
        let index_mirror = self.index(x2, y2);

        self.values[index] |= self.values[index_mirror];
		self.values[index_mirror] = false;
    }

    pub fn activate(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.values[index] = true;
    }

    pub fn visible_dots(&self) -> u32 {
        let mut count = 0;
        for x in 0..self.folded_columns {
            for y in 0..self.folded_rows {
                let index = self.index(x, y);
                if self.values[index] {count += 1}
            }
        }
        count
    }
}
