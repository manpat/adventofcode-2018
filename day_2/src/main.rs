type ID = [u8; 26];

fn str_to_id(s: &str) -> ID {
	let mut id = [0; 26];
	id.copy_from_slice(s.as_bytes());
	id
}


fn id_checkinfo(mut id: ID) -> (bool, bool) {
	id.sort();

	let mut elements = &id[..];
	let mut info = (false, false);

	while !elements.is_empty() {
		let num_duplicates = elements.iter().take_while(|&&x| x == elements[0]).count();

		match num_duplicates {
			2 => { info.0 = true }
			3 => { info.1 = true }
			_ => {}
		}

		if info.0 && info.1 { break }

		elements = &elements[num_duplicates..];
	}

	info
}


fn compare_ids(a: &ID, b: &ID) -> usize {
	a.iter().zip(b.iter())
		.filter(|(&a, &b)| a != b)
		.count()
}


fn main() {
	let input = include_str!("../input.txt");
	let ids = input.lines().map(str_to_id).collect::<Vec<_>>();

	let (two_sum, three_sum) = ids.iter().cloned()
		.map(id_checkinfo)
		.fold((0, 0), |(a2, a3), (x2, x3)| {
			(a2 + x2 as u32, a3 + x3 as u32)
		});

	println!("checksum {}", two_sum * three_sum);

	let mut ids_view = &ids[..];

	while !ids_view.is_empty() {
		let id_a = ids_view[0];
		ids_view = &ids_view[1..];

		if let Some(id_b) = ids_view.iter().find(|id_b| compare_ids(&id_a, id_b) == 1) {
			let common_parts = id_a.iter().zip(id_b.iter())
				.filter(|(&a, &b)| a == b)
				.map(|p| *p.0 as char)
				.collect::<String>();

			println!("similar id {}", common_parts);
			break;
		}
	}
}
