/// An iterator that produces the terms of the Fibonacci sequence, starting
/// at 0. Returns None on overflow.
///
/// # Examples
///
/// ```
/// use segtrs::FibonacciIterator;
/// let expected_terms: Vec<u64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21];
/// let produced_terms: Vec<u64> = FibonacciIterator::new()
/// 	.take(expected_terms.len())
/// 	.collect();
/// assert_eq!(expected_terms, produced_terms);
/// ```
pub struct FibonacciIterator {
	// This is always the next term to be returned, if any.
	f0: Option<u64>,
	// The value following f0, if any.
	f1: Option<u64>,
}

impl FibonacciIterator {
	pub fn new() -> Self {
		Self {
			f0: Some(0),
			f1: Some(1),
		}
	}
}

impl Iterator for FibonacciIterator {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		// Stop if f0 has overflown
		let result = self.f0?;

		let next = match (self.f0, self.f1) {
			(Some(f0_val), Some(f1_val)) => f0_val.checked_add(f1_val),
			_ => None,
		};

		self.f0 = self.f1;
		self.f1 = next;

		Some(result)
	}
}
