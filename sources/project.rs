use orbit::model::Shape;

use crate::camera::Camera;
use crate::lot::{Lot, Role};


#[derive(Debug)]
pub struct Project {
	pub cameras: Vec<Camera>,
	pub lots: Vec<Lot>,

	level: i8,
	levels: Vec<Vec<isize>>,
}


impl Project {
	pub fn new (mut lots: Vec<Lot>, shapes: &mut [Shape]) -> Option<Self> {
		let mut level = None;
		let mut heights = Vec::new();

		for lot in lots.iter_mut() {
			lot.process(shapes);

			if lot.role == Role::Living {
				heights.extend(lot.floors
					.iter()
					.map(|&index| (lot.building, level_height(&shapes[index]))));

				if lot.level < level.unwrap_or(i8::MAX) {
					level = Some(lot.level);
				}
			}
		}

		heights.sort_unstable();
		heights.dedup();

		let buildings = heights.last().map(|entry| (entry.0 + 1)).unwrap_or_default() as _;
		let mut iterator = heights.iter();
		let mut levels = vec![Vec::new(); buildings];

		while let Some((index, _)) = iterator.next() {
			levels[*index as usize].extend(iterator
				.by_ref()
				.take_while(|entry| entry.0 == *index)
				.map(|entry| entry.1));
		}

		Some(Self {
			cameras: Vec::new(),
			level: level.unwrap_or(0),
			levels,
			lots,
		})
	}

	pub fn lot_levels<'a> (&'a self, index: usize, shapes: &'a [Shape]) -> impl Iterator<Item = u8> + 'a {
		let lot = &self.lots[index];

		lot.floors
			.iter()
			.map(|&index| self.shape_level(lot.building, &shapes[index]))
	}

	#[inline]
	pub fn relative_level (&self, level: i8) -> u8 {
		let level = level - self.level;

		if level.is_negative() { 0 } else { level as _ }
	}

	pub fn shape_level (&self, building: u8, shape: &Shape) -> u8 {
		let mut height = level_height(shape);

		if !shape.is_downward_facing() {
			height -= 1;
		}

		let levels = &self.levels[building as usize];

		levels
			.iter()
			.position(|&level_height| level_height > height)
			.unwrap_or(levels.len()) as _
	}
}


#[inline]
fn level_height (shape: &Shape) -> isize {
	shape.center()[2].round() as _
}
