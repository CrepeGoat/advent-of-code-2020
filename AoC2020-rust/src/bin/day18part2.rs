use std::io::BufRead;
use std::ops::{Add, Mul};


fn could_be_expr<T>(s: &str) -> bool {
	let mut count_parens = 0isize;

	for c in s.chars() {
		match c {
			'(' => count_parens += 1,
			')' => {
				count_parens -= 1;
				if count_parens < 0 {
					return false
				}
			},
			_ => (),
		}
	}
	count_parens == 0
}

fn parse_expr<T>(s: &str) -> Result<T, ()>
where T: Copy + Add<Output = T> + Mul<Output = T> + std::str::FromStr
{
	// Short-circuit obviously wrong expressions
	if !could_be_expr::<T>(s) {
		return Err(());
	}

	// Expression in parentheses
	if &s[..1] == "(" && &s[s.len()-1..] == ")" {
		if let Ok(val) = parse_expr(&s[1..s.len()-1]) {
			return Ok(val);
		}
	}

	// Literal number
	if let Ok(val) = s.parse::<T>() {
		return Ok(val);
	}

	// Nested operations
	let op_len = " _ ".len();
	if s.len() < 2 + op_len {
		return Err(());
	}
	// Top-down approach -> consume lowest priority (*) first
	for (s1, _s2, s3) in (1..(s.len() - op_len)).rev()
		.map(|i| {
			let (s1, s23) = s.split_at(i);
			let (s2, s3) = s23.split_at(op_len);
			(s1, s2, s3)
		})
		.filter(|(_s1, s2, _s3)| s2 == &" * ")
	{
		if let Ok(lhs) = parse_expr::<T>(s1) {
			if let Ok(rhs) = parse_expr::<T>(s3) {
				return Ok(lhs * rhs);
			}
		}
	}
	// Top-down approach -> consume highest priority last
	for (s1, _s2, s3) in (1..(s.len() - op_len)).rev()
		.map(|i| {
			let (s1, s23) = s.split_at(i);
			let (s2, s3) = s23.split_at(op_len);
			(s1, s2, s3)
		})
		.filter(|(_s1, s2, _s3)| s2 == &" + ")
	{
		if let Ok(lhs) = parse_expr::<T>(s1) {
			if let Ok(rhs) = parse_expr::<T>(s3) {
				return Ok(lhs + rhs);
			}
		}
	}

	// Imparseable input
	Err(())
}


fn main() {
	let stream = std::io::BufReader::new(std::io::stdin());	
	let result = stream.lines().map(|r| r.unwrap())
		.map(|s| parse_expr::<u64>(&s[..]))
		.try_fold(0, |x, xi| xi.map(|y| x+y)).unwrap();

	println!("{:?}", result);
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse0() {
		assert_eq!(parse_expr::<u64>("1 + 2 * 3 + 4 * 5 + 6"), Ok(231));
	}

	#[test]
	fn parse1() {
		assert_eq!(parse_expr::<u64>("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
	}

	#[test]
	fn parse2() {
		assert_eq!(parse_expr::<u64>("2 * 3 + (4 * 5)"), Ok(46));
	}

	#[test]
	fn parse3() {
		assert_eq!(parse_expr::<u64>("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(1445));
	}

	#[test]
	fn parse4() {
		assert_eq!(
			parse_expr::<u64>("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
			Ok(669060),
		);
	}

	#[test]
	fn parse5() {
		assert_eq!(
			parse_expr::<u64>("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
			Ok(23340),
		);
	}

	#[test]
	fn parse6() {
		assert_eq!(
			parse_expr::<u64>("((3) * (2))"),
			Ok(6),
		);
	}
}
