use std::cell::RefCell;

use orbit::model::Shape;

use crate::camera::Camera;
use crate::lot::{Lot, Role};


#[derive(Debug)]
pub struct Project {
	pub cameras: Vec<Camera>,
	pub lots: Vec<Lot>,

	angles: Vec<Vec<f64>>,
	heights: Vec<Vec<isize>>,
	offset: i8,
	offsets: Vec<i8>,
}


impl Project {
	pub fn new (lots: Vec<Lot>, shapes: &mut [Shape]) -> Option<Self> {
		let mut buildings: Vec<_> = lots
			.iter()
			.filter(|lot| lot.role != Role::Circulation)
			.flat_map(|lot| lot.floors
				.iter()
				.zip(lot.levels.iter())
				.map(|(index, level)| (lot.building as usize, shape_height(&shapes[*index]), !lot.is_visible(), *level)))
			.collect();

		buildings.sort_unstable();
		buildings.dedup_by_key(|(building, height, ..)| (*building, *height));

		let length = buildings.last().map_or(0, |(building, ..)| building + 1);
		let mut heights = vec![Vec::new(); length];
		let mut offsets = vec![i8::MAX; length];
		let mut offset = None;

		for group in buildings.group_by(|group_a, group_b| group_a.0 == group_b.0) {
			let mut iterator = group.iter().skip_while(|(.., is_hidden, _)| *is_hidden);

			if let Some((.., level)) = iterator.next() {
				let index = group[0].0;

				heights[index].extend(iterator.map(|(_, height, ..)| *height));
				offsets[index] = *level;

				if !offset.is_some_and(|offset| offset <= *level) {
					offset = Some(*level);
				}
			}
		}

		Some(Self {
			angles: Vec::new(),
			cameras: Vec::new(),
			heights,
			lots,
			offset: offset.unwrap_or(0),
			offsets,
		})
	}

	#[inline]
	pub fn set_angles (&mut self, angles: Vec<RefCell<Vec<f64>>>) {
		self.angles.extend(angles.into_iter().map(RefCell::into_inner));
	}

	pub fn set_cameras (&mut self, cameras: Vec<Camera>) {
		if self.lots.is_empty() {
			let level = cameras
				.iter()
				.filter_map(|camera| if let Camera::Level { absolute, .. } = camera { Some(absolute) } else { None })
				.min();

			if let Some(level) = level {
				self.offset = *level;
			}
		}

		self.cameras = cameras;
	}

	#[inline]
	pub fn absolute_level (&self, level: u8) -> i8 {
		level as i8 + self.offset
	}

	#[inline]
	pub fn angle (&self, camera: usize, viewport: usize) -> Option<f64> {
		self.angles.get(camera)?.get(viewport).copied()
	}

	#[inline]
	pub fn relative_level (&self, level: i8) -> u8 {
		let level = level - self.offset;

		if level.is_negative() { 0 } else { level as _ }
	}

	pub fn shape_relative_level (&self, building: u8, shape: &Shape) -> u8 {
		let mut shape_height = shape_height(shape);

		if !shape.is_downward_facing() {
			shape_height -= 1;
		}

		let heights = &self.heights[building as usize];

		let level = heights
			.iter()
			.position(|&height| height > shape_height)
			.unwrap_or(heights.len()) as i8 + self.offsets[building as usize] - self.offset;

		if level.is_negative() { 0 } else { level as _ }
	}
}


#[inline]
fn shape_height (shape: &Shape) -> isize {
	shape.center()[2].round() as _
}
