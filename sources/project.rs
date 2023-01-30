use std::cell::RefCell;

use orbit::model::Shape;

use crate::camera::Camera;
use crate::lot::{Lot, Role};


#[derive(Debug)]
pub struct Project {
	pub cameras: Vec<Camera>,
	pub lots: Vec<Lot>,

	angles: Vec<Vec<f64>>,
	levels: Vec<Vec<isize>>,
	offset: i8,
	offsets: Vec<i8>,
}


impl Project {
	pub fn new (lots: Vec<Lot>, shapes: &mut [Shape]) -> Option<Self> {
		let mut buildings_heights = Vec::new();
		let mut buildings_offsets = Vec::with_capacity(lots.len());

		for lot in &lots {
			if lot.role == Role::Living && lot.name.is_some() {
				buildings_heights.reserve(lot.floors.len());
				buildings_offsets.push((lot.building, lot.level));

				for index in &lot.floors {
					buildings_heights.push((lot.building, level_height(&shapes[*index])));
				}
			}
		}

		buildings_heights.sort_unstable();
		buildings_offsets.sort_unstable();
		buildings_heights.dedup();

		let buildings = buildings_heights.last().map(|entry| entry.0 + 1).unwrap_or_default() as _;
		let mut offsets = vec![i8::MAX; buildings];

		for (index, level) in buildings_offsets {
			let offset = &mut offsets[index as usize];

			if level < *offset {
				*offset = level;
			}
		}

		let mut levels = vec![Vec::new(); buildings];
		let mut iterator = buildings_heights.iter().peekable();
		let length = buildings_heights.len();

		while let Some((index, _)) = iterator.next() {
			let levels = &mut levels[*index as usize];

			levels.reserve(length);

			while let Some((_, height)) = iterator.next_if(|(building, _)| building == index) {
				levels.push(*height)
			}

			levels.shrink_to_fit();
		}

		Some(Self {
			angles: Vec::new(),
			cameras: Vec::new(),
			levels,
			lots,
			offset: offsets.iter().min().copied().unwrap_or(0),
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
		let mut height = level_height(shape);

		if !shape.is_downward_facing() {
			height -= 1;
		}

		let levels = &self.levels[building as usize];

		let level = levels
			.iter()
			.position(|&level_height| level_height > height)
			.unwrap_or(levels.len()) as i8 + self.offsets[building as usize] - self.offset;

		if level.is_negative() { 0 } else { level as _ }
	}
}


#[inline]
fn level_height (shape: &Shape) -> isize {
	shape.center()[2].round() as _
}
