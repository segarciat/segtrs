use std::error::Error;
use std::collections::BTreeSet;

/// Determines whether `n` is prime.
///
/// # Examples
///
/// ```
/// assert!(!segtrs::numt::is_prime(1));
/// assert!(segtrs::numt::is_prime(2));
/// assert!(!segtrs::numt::is_prime(4));
/// ```
pub fn is_prime(n: u64) -> bool {
	if n == 2 {
		return true;
	}
	if n < 2 || (n % 2) == 0 {
		return false;
	}

	let mut k = 3;
	while (k * k) <= n {
		if (n % k) == 0 {
			return false;
		}
		k += 1;
	}

	true
}

/// Computes the $n$th triangular number according to the formula
/// $t_n = \frac{n(n+1)}{2}. On overflow, returns an error.
///
/// # Examples
///
/// ```
/// let t_5 = segtrs::numt::triangular_number(5).unwrap();
/// assert_eq!(15, t_5);
///
/// assert!(segtrs::numt::triangular_number(u64::MAX).is_err());
/// ```
pub fn triangular_number(n: u64) -> Result<u64, Box<dyn Error>> {
	let n_plus_1 = n.checked_add(1).ok_or_else(|| "overflow")?;
	let t_n = n_plus_1.checked_mul(n).ok_or_else(|| "overflow")? / 2;
	Ok(t_n)
}

/// Produces all the factors of `n`. Uses the convention that $0$ is the only
/// factors of $0$.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeSet;
/// 
/// let factors = segtrs::numt::factors_of(12);
/// assert_eq!(BTreeSet::from([1, 12, 2, 6, 3, 4]), factors);
/// ```
pub fn factors_of(n: u64) -> BTreeSet<u64> {
	if n < 2 {
		return BTreeSet::from([n]);
	}

	let mut factors = BTreeSet::new();
	let sqrt = n.isqrt();
	
	for k in 1..=sqrt {
		let (q, r) = (n / k, n % k);
		if r == 0 {
			factors.insert(k);
			if q != sqrt {
				factors.insert(q);
			}
		}
	}
	
	factors
}

/// Determines whether `s` is a palindrome. Ignores non-alphaumeric characters,
/// and ignores case sensitivity.
///
/// # Examples
///
/// ```
/// assert!(segtrs::numt::is_palindrome("Taco Cat"));
/// assert!(segtrs::numt::is_palindrome("1234321"));
/// assert!(!segtrs::numt::is_palindrome("kyoto"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
	let chars: Vec<char>= s.to_lowercase().chars().collect();
	let mut left_idx = 0;
	let mut right_idx = chars.len() - 1;
	while left_idx < right_idx {
		let left_char = &chars[left_idx];
		if !left_char.is_alphanumeric() {
			left_idx += 1;
			continue;
		}

		let right_char = &chars[right_idx];
		if !right_char.is_alphanumeric() {
			right_idx -= 1;
			continue;
		}

		if left_char != right_char {
			return false;
		}
		left_idx += 1;
		right_idx -= 1;
	}

	true
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn below_2_are_not_prime() {
		assert!(!is_prime(0));
		assert!(!is_prime(1));
	}

	#[test]
	fn primes_below_20() {
		assert!(is_prime(2));
		assert!(is_prime(3));
		assert!(is_prime(5));
		assert!(is_prime(7));
		assert!(is_prime(11));
		assert!(is_prime(13));
		assert!(is_prime(17));
		assert!(is_prime(19));
	}

	#[test]
	fn palindrome_one_casing() {
		assert!(is_palindrome("tacocat"));
	}

	#[test]
	fn palindrome_case_insensitive() {
		assert!(is_palindrome("TacoCat"));
	}

	#[test]
	fn palindrome_with_spaces() {
		assert!(is_palindrome("taco cat"));
	}

	#[test]
	fn palindrome_numbers() {
		assert!(is_palindrome("1234321"));
	}

	#[test]
	fn triangular_small() {
		assert_eq!(0, triangular_number(0).unwrap());
		assert_eq!(1, triangular_number(1).unwrap());
		assert_eq!(3, triangular_number(2).unwrap());
		assert_eq!(6, triangular_number(3).unwrap());
		assert_eq!(10, triangular_number(4).unwrap());
		assert_eq!(15, triangular_number(5).unwrap());
	}

	#[test]
	fn triangular_overflow() -> Result<(), String> {
		let result = triangular_number(u64::MAX);
		if result.is_err() {
			Ok(())
		} else {
			Err(String::from("large triangular numbers should overflow"))
		}
	}

	#[test]
	fn factors_of_zero_and_one() {
		assert_eq!(BTreeSet::from([0]), factors_of(0));
		assert_eq!(BTreeSet::from([1]), factors_of(1));
	}

	#[test]
	fn factors_non_square() {
		let result = factors_of(28);
		assert_eq!(BTreeSet::from([1, 2, 4, 7, 14, 28]), result);
	}
	
	#[test]
	fn factors_of_a_square() {
		let result = factors_of(64);
		assert_eq!(BTreeSet::from([1, 2, 4, 8, 16, 32, 64]), result);
	}
}
