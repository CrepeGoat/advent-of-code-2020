use std::io::{BufRead, BufReader};
use std::collections::HashSet;


fn parse_input<R: BufRead>(reader: R) -> HashSet<(isize, isize, isize)> {
	reader.lines()
		.filter_map(|r| r.ok())
		.enumerate()
		.flat_map(|(i, s)|
			s.chars().enumerate()
				.filter(|(_j, ch)| *ch == '#')
				.map(move |(j, _ch)| (i as isize, j as isize, 0isize))
				.collect::<Vec<_>>().into_iter()
		)
		.collect()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_input() {
		let eg_reader = BufReader::new(".#.\n..#\n###\n".as_bytes());

		assert_eq!(
			parse_input(eg_reader),
			vec![
				(0, 1, 0),
				(1, 2, 0),
				(2, 0, 0),
				(2, 1, 0),
				(2, 2, 0),
			].into_iter().collect(),
		);
	}
}
