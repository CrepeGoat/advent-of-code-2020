use std::convert::TryInto;
use std::str::FromStr;
use std::io::BufRead;


fn parsing_input<R: BufRead, T: FromStr>(reader: R) -> impl Iterator<Item=T> {
	reader.lines()
		.filter_map(|r| r.ok())
		.filter_map(|s| s.parse::<T>().ok())
}

fn iter_sum_triplets<'a>(seq: &'a mut Vec<u32>, value: u32) -> impl Iterator<Item=(u32, u32, u32)> + 'a
{
	let bin_count = (seq.len() as f64).sqrt() as u32;
	let sort_key = {|a: &u32| (*a % bin_count, *a)};

	seq.sort_unstable_by_key(sort_key);
	let mut seq_bins: Vec<&[u32]> = Vec::new();
	seq_bins.reserve_exact(bin_count as usize);
	{
		let mut seq_ref = &seq[..];
		for n in 1u32..=bin_count {
			let i = seq_ref.binary_search_by_key(&(n, 0u32), sort_key)
				.map_or_else(|x| x, |x| x);
			let (seq_i, seq_suffix) = seq_ref.split_at(i);
			seq_bins.push(seq_i);
			seq_ref = seq_suffix;
		}
		if !seq_ref.is_empty() {panic!("{:?} unbinned elements", seq_ref.len());}
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
				(0..bins.1.len())
					.map(move |j| (i, j))
					.take_while(|(i, j)| bins.0[*i] + bins.1[*j] < value)
			)
			.find_map(|(i, j)|
				bins.2
					.binary_search(&(value-bins.0[i]-bins.1[j]))
					.ok()
					.map(|k| (bins.0[i], bins.1[j], bins.2[k]))
			)
	}

	let seq_bins_len = seq_bins.len();
	(0usize..seq_bins_len)
	.flat_map(
		move |i|
		(i+1..seq_bins_len)
			.map(move |j| (i, j))
	)
	.filter_map(
		move |(i, j)|
		(i + j)
			.try_into().ok()
			.and_then(|ipj| value.checked_sub(ipj))
			.map(|k| (i, j, (k % bin_count) as usize))
	)
	.filter_map(
		move |(i, j, k)|
		search_bins(
			value, (seq_bins[i], seq_bins[j], seq_bins[k])
		)
	)
}

fn main() {
	println!("Enter input sequence: ");
	let stdin = std::io::stdin();
	let mut parsed_inputs: Vec<u32> = parsing_input(stdin.lock()).collect();

	let vals = iter_sum_triplets(&mut parsed_inputs, 2020).next().unwrap();
	println!("triplet summing to 2020: {:?}, {:?}, {:?}", vals.0, vals.1, vals.2);
	println!("product: {:?}", vals.0 * vals.1 * vals.2);
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_find_sum_pair() {
		let mut sequence = vec![1721, 979, 366, 299, 675, 1456];
		assert_eq!(iter_sum_triplets(&mut sequence, 2020).next(), Some((366, 675, 979)));
	}
}
