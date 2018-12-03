#![feature(box_syntax)]

const FABRIC_DIMS: usize = 1000;

#[derive(Debug, Copy, Clone)]
struct Rect {
	id: usize,
	x: usize, y: usize,
	w: usize, h: usize,
}

impl Rect {
	fn from_str(s: &str) -> Rect {
		let mut it = s.split(|c: char| !c.is_numeric())
			.filter(|s| !s.is_empty())
			.filter_map(|s| s.parse().ok());

		Rect {
			id: it.next().unwrap(),
			x: it.next().unwrap(),
			y: it.next().unwrap(),
			w: it.next().unwrap(),
			h: it.next().unwrap(),
		}
	}

	fn iter(self) -> impl Iterator<Item=usize> {
		let x2 = self.x + self.w;
		let y2 = self.y + self.h;

		(self.y .. y2)
			.flat_map(move |y| {
				let row = y * FABRIC_DIMS;
				self.x + row .. x2 + row
			})
	}
}



struct Fabric {
	cells: [u32; FABRIC_DIMS*FABRIC_DIMS],
}

impl Fabric {
	fn new() -> Self {
		Fabric{ cells: [0; FABRIC_DIMS*FABRIC_DIMS] }
	}

	fn claim(&mut self, r: &Rect) {
		for i in r.iter() {
			self.cells[i] += 1;
		}
	}

	fn num_conflicting_cells(&self) -> usize {
		self.cells.iter()
			.filter(|&&x| x > 1)
			.count()
	}

	fn is_uncontested(&self, r: &Rect) -> bool {
		r.iter().map(|i| self.cells[i])
			.all(|c| c == 1)
	}
}


fn main() {
	let input = include_str!("../input.txt");
	let rects: Vec<_> = input.lines().map(Rect::from_str).collect();
	let mut fabric = box Fabric::new();

	for r in rects.iter() {
		fabric.claim(r);
	}

	println!("conflicting claims: {} in²", fabric.num_conflicting_cells());

	let uncontested_claim = rects.iter()
		.find(|&r| fabric.is_uncontested(r))
		.unwrap();

	println!("uncontested_claim: #{}", uncontested_claim.id);
}
