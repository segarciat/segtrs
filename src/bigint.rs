/// Represents a base-10 number that can have any number of digits.
#[derive(Debug)]
pub struct BigInt {
	digits: Vec<u8>,
}

impl BigInt {
	/// Create a BigInt from an iterator. Expects the least-significant digit
	/// to appear first.
	///
	/// # Examples
	///
	/// ```
	/// use segtrs::BigInt;
	/// // Represents decimal number 314
	/// let digits = vec![4, 1, 3];
	/// 
	/// let mut bigint = BigInt::new(digits.into_iter());
	/// assert_eq!(&vec![4, 1, 3], bigint.digits());
	/// ```
	///
	pub fn new(it: impl Iterator<Item = u8>) -> Self {
		let mut digits: Vec<u8> = it.collect();
		for d in &digits {
			if *d > 9 {
				panic!("only digits 0 through 9 allowed");
			}
		}
		// Eliminate non-essential leading zeros
		while digits.len() > 0 && *digits.last().unwrap() == 0 {
			digits.pop();
		}

		BigInt {
			digits: if digits.len() > 0 { digits } else { vec![0] },
		}
	}

	/// Obtain a references to the digits stored by the BigInt object.
	pub fn digits(&self) -> &Vec<u8> {
		&self.digits
	}

	/// Produce a new BigInt object who digits correspond to the digits of the
	/// sum of the number represented by `self` and `other`.
	///
	/// # Examples
	///
	/// ```
	/// use segtrs::BigInt;
	/// // Represents the number decimal 31
	/// let a = BigInt::new(vec![1, 3, 0].into_iter());
	/// // Represents the number decimal 987
	/// let b = BigInt::new(vec![7, 8, 9, 1].into_iter());
	/// // Represents the sum of 31 and 987, which is 1018
	/// let sum = a.add(&b);
	/// assert_eq!(&vec![1, 3], a.digits());
	/// assert_eq!(&vec![7, 8, 9, 1], b.digits());
	/// assert_eq!(&vec![8, 1, 0, 2], sum.digits());
	/// ```
	pub fn add(&self, other: &BigInt) -> Self {
		let mut result = vec![];

		// Add digit-by-digit, pad shorter number with zeros
		let mut carry = 0;
		let max_len = self.digits.len().max(other.digits.len());
		for i in 0..max_len {
			let mut temp = carry;
			temp += if i < self.digits.len() { self.digits[i] } else { 0 };
			temp += if i < other.digits.len() { other.digits[i] } else { 0 };

			let digit = temp % 10;
			carry = temp / 10;
			result.push(digit);
		}

		while carry > 0 {
			let digit = carry % 10;
			carry = carry / 10;
			result.push(digit);
		}

		BigInt {
			digits: result,
		}
	}

	pub fn multiply(&self, other: &BigInt) -> Self {
		let mut products = vec![];

		for (num_zeros, a) in self.digits().iter().enumerate() {
			let mut single_digit_product = vec![];
			for _ in 0..num_zeros {
				single_digit_product.push(0);
			}

			// Multiply a by every digit of other
			let mut carry = 0;
			for b in &other.digits {
				let p = a * b + carry;
				single_digit_product.push(p % 10);
				carry = p / 10;
			}

			// Exhaust the carry that remais, if any
			while carry != 0 {
				single_digit_product.push(carry % 10);
				carry /= 10;
			}
			products.push(single_digit_product);
		}
		// Add all the products
		let mut result = BigInt::new(vec![].into_iter());
		for product in products.into_iter() {
			let bigint = BigInt::new(product.into_iter());
			result = result.add(&bigint);
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic(expected = "only digits 0 through 9")]
	fn bigint_with_invalid_decimal_digits() {
		BigInt::new(vec![7, 8, 9, 10].into_iter());
	}

	#[test]
	fn bigint_removes_extra_zeros() {
		let bigint = BigInt::new(vec![7, 8, 9, 0, 0, 0].into_iter());
		assert_eq!(&vec![7, 8, 9], bigint.digits());
	}

	#[test]
	fn big_all_zeros_equals_one_zero() {
		let bigint = BigInt::new(vec![0, 0, 0].into_iter());
		assert_eq!(&vec![0], bigint.digits());
	}

	#[test]
	fn bigint_add_two() {
		// Add 25 and 98 to get 123
		let a = BigInt::new(vec![5, 2].into_iter());
		let b = BigInt::new(vec![8, 9].into_iter());
		let sum = a.add(&b);

		assert_eq!(&vec![3, 2, 1], sum.digits());
	}

	#[test]
	fn bigint_multiply_single_digit() {
		let a = BigInt::new(vec![3].into_iter());
		let b = BigInt::new(vec![2].into_iter());
		let product = a.multiply(&b);

		assert_eq!(&vec![6], product.digits());
	}

	#[test]
	fn bigint_multiply_multi_digit() {
		// 12 multiplied by 345 is 4140
		let a = BigInt::new(vec![2, 1].into_iter());
		let b = BigInt::new(vec![5, 4, 3].into_iter());
		let product = a.multiply(&b);

		assert_eq!(&vec![0, 4, 1, 4], product.digits());
	}
}
