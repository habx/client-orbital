use std::mem::swap;

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
		let mut heights = Vec::new();
		let mut level = None;

		for lot in lots.iter_mut() {
			lot.process(shapes);

			if lot.role == Role::Living && lot.name.is_some() {
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

		let buildings = heights.last().map(|entry| entry.0 + 1).unwrap_or_default() as _;
		let mut iterator = heights.iter().peekable();
		let length = heights.len();
		let mut levels = vec![Vec::new(); buildings];

		while let Some((index, _)) = iterator.next() {
			let levels = &mut levels[*index as usize];

			levels.reserve(length);

			while let Some((_, height)) = iterator.next_if(|(building, _)| building == index) {
				levels.push(*height)
			}

			levels.shrink_to_fit();
		}

		Some(Self {
			cameras: Vec::new(),
			level: level.unwrap_or(0),
			levels,
			lots,
		})
	}

	pub fn set_cameras (&mut self, cameras: &mut Vec<Camera>) {
		if self.lots.is_empty() {
			let level = cameras
				.iter()
				.filter_map(|camera| if let Camera::Level { absolute, .. } = camera { Some(absolute) } else { None })
				.min();

			if let Some(level) = level {
				self.level = *level;
			}
		}

		swap(&mut self.cameras, cameras);
	}

	#[inline]
	pub fn absolute_level (&self, level: u8) -> i8 {
		level as i8 + self.level
	}

	#[inline]
	pub fn relative_level (&self, level: i8) -> u8 {
		let level = level - self.level;

		if level.is_negative() { 0 } else { level as _ }
	}

	pub fn shape_relative_level (&self, building: u8, shape: &Shape) -> u8 {
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
