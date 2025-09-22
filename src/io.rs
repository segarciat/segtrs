use std::error::Error;
use std::fs::File;
use std::io::BufReader;
// BufReader implements the BufRead trait for its lines() method.
// To use lies(), BufRead must be in scope.
use std::io::BufRead;

pub fn load_number_grid(filepath: &str)
	-> Result<Vec<Vec<u64>>, Box<dyn Error>> {

	let file = File::open(filepath)?;
	let reader = BufReader::new(file);

	// Read squared grid of itnegers
	let mut grid = vec![];
	for line in reader.lines() {
		let mut row = vec![];
		for s in line?.split_whitespace() {
			let n: u64 = s.parse()?;
			row.push(n);
		}
		grid.push(row);
	}
	Ok(grid)
}
