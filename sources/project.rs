use orbit::model::Shape;

use crate::camera::Camera;
use crate::lot::{Lot, Role};


#[derive(Debug)]
pub struct Project {
	pub cameras: Vec<Camera>,
	pub lots: Vec<Lot>,

	level: i8,
	levels: Vec<(u8, isize)>,
}


impl Project {
	pub fn new (mut lots: Vec<Lot>, shapes: &mut [Shape]) -> Option<Self> {
		let mut level = 0;

		for lot in lots.iter_mut() {
			lot.process(shapes);

			if lot.role == Role::Living && lot.level < level {
				level = lot.level;
			}
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
			cameras: Vec::new(),
			level,
			levels,
			lots,
		})
	}

	#[inline]
	pub fn relative_level (&self, level: i8) -> u8 {
		let level = level - self.level;

		if level.is_negative() { 0 } else { level as _ }
	}

	pub fn shape_level (&self, building: u8, shape: &Shape) -> u8 {
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
