use orbit::model::Shape;

use crate::lot::{Lot, Role};


#[derive(Debug)]
pub struct Project {
	pub lots: Vec<Lot>,

	levels: Vec<(u8, isize)>,
}


impl Project {
	pub fn new (mut lots: Vec<Lot>, shapes: &mut [Shape]) -> Option<Self> {
		for lot in &mut lots {
			lot.process(shapes);
		}

		let mut levels: Vec<_> = lots
			.iter()
			.filter(|lot| lot.role == Role::Living)
			.flat_map(|lot| lot.floors
				.iter()
				.map(|&index| (lot.building, level_height(&shapes[index]))))
			.collect();

		levels.sort_unstable();
		levels.dedup();

		Some(Self {
			levels,
			lots,
		})
	}

	pub fn level (&self, building: u8, shape: &Shape) -> u8 {
		let mut height = level_height(shape);

		if shape.normal()[2].is_sign_positive() {
			height -= 1;
		}

		self.levels
			.iter()
			.filter(|level| level.0 == building)
			.position(|level| level.1 >= height)
			.unwrap_or_default() as _
	}
}


#[inline]
fn level_height (shape: &Shape) -> isize {
	shape.center()[2].round() as _
}
