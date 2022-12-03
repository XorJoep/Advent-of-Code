pub struct Solver {
	expected_result: u64,
	solve_f: fn(&str) -> u64,
	input: &str,
	ex_input: &str
}

impl Solver {
	pub fn new(er: u64, ps: fn(&str) -> u64, inp: &str, ex_inp: &str) -> Solver {
		Solver {
            expected_result: er,
			solve_f: ps,
			input: inp,
			ex_input: ex_inp
        }
	}

	pub fn execute(input: &str) -> u64 {
		let start = Instant::now();
		let result = self.solve_f(input);

		println!("\tFinished after {:?}", start.elapsed());
	    println!("\tSolution found: {}", result);

	    result
	}

	pub fn solve_example() -> bool {
		println!("Example:");
		let result = self.execute(self.ex_input);

	    if self.expected_result == 0 {
	        println!("\tSkipping check");
	        true
	    }
	    else if result != self.expected_result {
	        println!("\tINCORRECT: Expected: [{}] but got [{}]", self.expected_result, result);
	        false
	    }
	    else {
	        println!("\tCORRECT");
	        true
	    }
	}

	pub fn solve_part() -> bool {
		solve_example() && execute(self.input)
	}
}