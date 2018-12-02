use std::collections::BTreeSet;

fn main() {
	let f = include_str!("../input.txt");
	let deltas = f.lines()
		.map(str::parse::<i32>)
		.map(Result::unwrap)
		.collect::<Vec<_>>();

	println!("final: {}", deltas.iter().sum::<i32>());

	let mut freq = 0;
	let mut seen_freqs = BTreeSet::new();
	seen_freqs.insert(0);

	loop {
		for d in deltas.iter() {
			freq += d;
			
			if !seen_freqs.insert(freq) {
				println!("first duplicate: {}", freq);
				return;
			}
		}
	}
}