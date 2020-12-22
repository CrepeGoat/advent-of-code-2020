#[macro_use] extern crate itertools;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};


type Coords = (isize, isize, isize);


fn parse_input<R: BufRead>(reader: R) -> HashSet<Coords> {
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


fn neighbors(coords: &Coords) -> impl Iterator<Item=Coords> {
	let i_vals = [coords.0, coords.0-1, coords.0+1];
	let j_vals = [coords.1, coords.1-1, coords.1+1];
	let k_vals = [coords.2, coords.2-1, coords.2+1];

	let iter = itertools::iproduct!(
		i_vals.iter(), j_vals.iter(), k_vals.iter()
	).skip(1);  // discard center of cube
	iter.map(|(&i, &j, &k)| (i, j, k)).collect::<Vec<_>>().into_iter()
}


fn next_state(is_active: bool, active_neighbors_count: usize) -> bool {
	match (is_active, active_neighbors_count) {
		(true, 2) => true,
		(_, 3) => true,
		_ => false,
	}
}


fn simulate_step(active_coords: HashSet<Coords>) -> HashSet<Coords> {
	let mut neighbor_counts = HashMap::<Coords, usize>::new();
	for coords in active_coords.iter().flat_map(neighbors) {
		let count = neighbor_counts.entry(coords).or_insert(0);
		*count += 1;
	}

	neighbor_counts.drain()
		.filter(|(coords, ncount)| next_state(active_coords.contains(coords), *ncount))
		.map(|(coords, _ncount)| coords)
		.collect()
}

fn run_simulation(mut active_coords: HashSet<Coords>) -> HashSet<Coords> {
	for _i in 0..6 {
		active_coords = simulate_step(active_coords);
	}
	active_coords
}


fn main() {
	println!("Enter input sequence: ");
	let stdin = std::io::stdin();
	let parsed_inputs = parse_input(stdin.lock());

	let result = run_simulation(parsed_inputs);
	println!("{:?}", result.len());
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
