#![allow(dead_code)]
#![allow(unused_variables)]


type Point<T> = (T, T);


pub struct Matrix {
	values: Vec<u32>,
	rows: usize,
	columns: usize
}

impl Matrix {
	pub fn new(input: &str) -> Matrix {
		Matrix {
            rows: input.lines().count(),
            columns: input.lines().nth(0).unwrap().chars().count(),
            values: input.lines().flat_map(|line| line.chars().map(|c| c.to_digit(10).expect("Not an int"))).collect::<Vec<u32>>()
        }
	}

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn print(&self) {
        println!("------------");
        for r in 0..self.rows {
            let index = r * self.columns;
            println!("{:?}", &self.values[index..index+self.columns]);
        }
        println!("------------");
    }

    fn index(&self, r: usize, c: usize) -> usize {
		self.columns * r + c
	}

    fn get(&self, r: usize, c: usize) -> u32 {
		let index = self.index(r, c);
		self.values[index]
	}

    fn incr(&mut self, r: usize, c: usize) {
		let index = self.index(r, c);
        self.values[index] += 1;
        
        if self.values[index] == 10 {
            self.incr_neighbours(r, c);
        }
            
	}

    pub fn incr_all(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.columns {
                self.incr(r, c)
            }
        }
    }

    pub fn reset_all(&mut self) -> usize {
        let mut flashes = 0;
        for r in 0..self.rows {
            for c in 0..self.columns {
                let index = self.index(r, c);
                if self.values[index] > 9 {
                    self.values[index] = 0;
                    flashes += 1;
                } 
            }
        }
        flashes
    }

    fn get_neighbours(&self, r: usize, c: usize) -> Vec<Point<usize>> {
        let mut ret = Vec::new();
        if r + 1 < self.rows {
            ret.push((r+1, c));
        }
        if r > 0 {
            ret.push((r-1, c));
        }
        if c + 1 < self.columns {
            ret.push((r, c+1));
        }
        if c > 0 {
            ret.push((r, c-1));
        }
        ret
    }

    fn incr_neighbours(&mut self, row_idx: usize, col_idx: usize) {
        self.neighbours_of_radius(row_idx, col_idx, 1).iter().for_each(|&(r, c)| self.incr(r,c))
    }

    fn neighbours_of_radius(&self, row_idx: usize, col_idx: usize, radius: usize) -> Vec<Point<usize>> {
        //input
        //array : 2D float64/int : data array to find the nearest neighours from
        //row_idx : int : row index for the center point for which nearest neighour needs to be searched
        //col_idx : int : index for the center point for which nearest neighour needs to be searched
        
        //output
        //returns a list
        //index of the nearest neighours
        //value at that cell
        
        //i iterates over row
        //j iterates over column
        
        let mut above_row = row_idx + radius +1; //defines the higher limt of row iterator
        if above_row + 1 > self.rows { //takes into account the array length to avoid crossing index limits
            above_row = self.rows
        }
        
        let below_row =  //defines lower limit
        if row_idx > radius //takes into account the zero index
            { row_idx - radius }
        else { 0 };
            
        let mut above_col = col_idx + radius+1; //defines the higher limit of column iterator
        if above_col + 1 > self.columns { //takes the end index into account
            above_col = self.columns
        }
        
        let below_col =  //defines the lower limit of column iterator
        if col_idx > radius //takes zero index into account
            { col_idx - radius }
        else { 0 };
        
        let mut indices = Vec::new();
        (below_row..above_row).for_each(|r|
            (below_col..above_col).for_each(|c|
                if !(r == row_idx && c == col_idx) {
                    indices.push((r,c))
                }
            )
        );               
        indices
    }
}
