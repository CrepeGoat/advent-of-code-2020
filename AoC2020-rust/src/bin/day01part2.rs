use std::str::FromStr;
use std::io::BufRead;


fn parsing_input<R: BufRead, T: FromStr>(reader: R) -> impl Iterator<Item=T> {
	reader.lines()
		.filter_map(|r| r.ok())
		.filter_map(|s| s.parse::<T>().ok())
}

fn find_sum_triplet(mut seq: Vec<u32>, value: u32) -> Option<(u32, u32, u32)>
{
	let bin_count = (seq.len() as f64).sqrt() as u32;
	let sort_key = {|a: &u32| (*a % bin_count, *a)};

	seq.sort_unstable_by_key(sort_key);
	let mut seq_bins: Vec<&[u32]> = Vec::new();
	seq_bins.reserve_exact(bin_count as usize);
	for n in 0..bin_count {
		let i = seq[..].binary_search_by_key(&(n as u32, n as u32), sort_key)
			.map_or_else(|x| x, |x| x);
		let j = seq[..].binary_search_by_key(&(n as u32 + 1, n as u32 + 1), sort_key)
			.map_or_else(|x| x, |x| x);
		seq_bins.push(&seq[i..j])
	}

	fn search_bins(value: u32, bins: (&[u32], &[u32], &[u32])) -> Option<(u32, u32, u32)>{
		if bins.0.len() > bins.2.len() {
			return search_bins(value, (bins.2, bins.1, bins.0));
		}
		if bins.1.len() > bins.2.len() {
			return search_bins(value, (bins.2, bins.0, bins.1));
		}
		if bins.0.len() > bins.1.len() {
			return search_bins(value, (bins.1, bins.0, bins.2));
		}

		(0usize..bins.0.len())
			.flat_map(|i|
				(i+1..bins.1.len())
					.map(move |j| (i, j))
					.take_while(|(i, j)| bins.0[*i] + bins.1[*j] < value)
			)
			.find_map(|(i, j)|
				bins.2[j+1..]
					.binary_search(&(value-bins.0[i]-bins.1[j]))
					.ok()
					.map(|k| (bins.0[i], bins.1[j], bins.2[k+j+1]))
			)
	}

	(0usize..seq_bins.len())
		.flat_map(|i|
			(i+1..seq_bins.len())
				.map(move |j| (i, j))
				.take_while(|(i, j)| seq[*i] + seq[*j] < value)
		)
		.find_map(|(i, j)| search_bins(value, (
			seq_bins[i],
			seq_bins[j],
			seq_bins[(value-(i as u32)-(j as u32) % bin_count) as usize]
		)))
}

fn main() {
	println!("Enter input sequence: ");
	let stdin = std::io::stdin();
	let parsed_inputs: Vec<u32> = parsing_input(stdin.lock()).collect();

	let vals = find_sum_triplet(parsed_inputs, 2020).unwrap();
	println!("triplet summing to 2020: {:?}, {:?}, {:?}", vals.0, vals.1, vals.2);
	println!("product: {:?}", vals.0 * vals.1 * vals.2);
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_find_sum_pair() {
		let sequence = vec![1721, 979, 366, 299, 675, 1456];
		assert_eq!(find_sum_triplet(sequence, 2020), Some((366, 675, 979)));
	}
}
