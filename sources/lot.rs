use std::ops::Range;

use orbit::model::Shape;
use orbit::utils::{center, dot_product, square_distance};


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
	Annex,
	Circulation,
	Living,
}


#[derive(Debug)]
pub struct Lot {
	pub building: u8,
	pub floors: Vec<usize>,
	pub identifier: String,
	pub images: Vec<(u8, String)>,
	pub level: i8,
	pub levels: Vec<i8>,
	pub name: Option<String>,
	pub range: Range<usize>,
	pub role: Role,
	pub slug: Option<String>,
	pub surface_area: Option<u64>,
	pub typology: Option<u8>,
}


impl Lot {
	pub fn new (
		start: usize,
		shapes: &mut [Shape],
		identifier: String,
		images: Vec<(u8, String)>,
		levels: Vec<String>,
		name: Option<String>,
		slug: Option<String>,
		surface_area: Option<u64>,
		typology: Option<u8>,
	) -> Result<Self, String> {
		let value = identifier.to_lowercase();

		if let Some(((building, level), role)) = parse_building(&value).zip(parse_level(&value)).zip(Role::parse(&value)) {
			let end = shapes.len();
			let mut levels = parse_levels(levels);
			let mut floors = Vec::with_capacity(levels.len());

			if role != Role::Circulation {
				floors.push(start);
				shapes[start..].sort_unstable_by_key(|shape| (shape.center()[2] * 10_000.) as i64);

				floors.extend(shapes[start..end - 2]
					.windows(3)
					.enumerate()
					.filter_map(|(index, window)| (window[0].is_vertical() && window[1].is_horizontal() && window[2].is_horizontal())
						.then_some(start + index + 2)));

				if floors.len() != levels.len() {
					#[cfg(test)]
					if role == Role::Living {
						if levels.is_empty() {
							if floors.len() != 1 {
								eprintln!("  {identifier} detected {} levels but none declared", floors.len());
							}
						} else {
							eprintln!("  {identifier} detected {} levels out of the {} declared", floors.len(), levels.len());
						}
					}

					levels.resize(floors.len(), level);
				}

				for index in 0..floors.len() {
					let start = floors[index];
					let end = floors.get(index + 1).map_or(end - 1, |value| value - 1);

					if let Some(first_wall) = shapes[start..end].iter().position(|shape| shape.is_vertical()) {
						let (floors, walls) = shapes[start..end].split_at_mut(first_wall);

						#[cfg(test)]
						if floors.is_empty() {
							eprintln!("  {identifier} no floor");
							continue
						}

						let center = center(&walls.iter().map(Shape::center).collect::<Vec<_>>());

						for shape in walls {
							#[cfg(test)]
							if shape.is_horizontal() {
								eprintln!("  {identifier} floor among walls");
							}

							if dot_product(center - shape.center(), shape.normal()).is_sign_positive() {
								shape.flip();
							}
						}

						floors.sort_unstable_by_key(|shape| (square_distance(shape.center(), center) * 10_000.) as i64);

						for shape in floors {
							// `is_height_positive` is a relaxed version of the `is_upward_facing` method.
							// It allows other not-a-wall shapes, e.g. stairs, to be displayed on sectional views.
							if shape.is_height_positive() {
								shape.flip();
							}
						}

						// Ceiling
						let ceiling = &mut shapes[end];

						#[cfg(test)]
						if !ceiling.is_horizontal() {
							eprintln!("  {identifier} no ceiling");
						}

						if ceiling.is_downward_facing() {
							ceiling.flip();
						}
					} else {
						#[cfg(test)]
						eprintln!("  {identifier} no walls");
					}
				}
			}

			Ok(Self {
				building,
				floors,
				identifier,
				images,
				level,
				levels,
				name,
				range: start..end,
				role,
				slug,
				surface_area,
				typology,
			})
		} else {
			Err(identifier)
		}
	}

	#[inline]
	pub fn is_visible (&self) -> bool {
		self.role == Role::Living && self.name.is_some()
	}

	#[inline]
	pub fn class (&self) -> String {
		format!("lot{}", self.typology.map_or_else(String::new, |typology| format!(" t{}", typology)))
	}
}

impl Role {
	#[inline]
	fn parse (value: &str) -> Option<Self> {
		Some(match value[value.find(['c', 'h', 'l'])?..].chars().next()? {
			'c' => Self::Circulation,
			'h' => Self::Annex,
			'l' => Self::Living,
			_ => unreachable!()
		})
	}
}


#[inline]
fn parse_building (value: &str) -> Option<u8> {
	let value = &value[value.find('b')?..];

	Some(value.split(char::is_alphabetic).nth(1)?.parse::<u8>().ok()? - 1)
}

#[inline]
fn parse_level (value: &str) -> Option<i8> {
	let value = &value[value.find(['e', 's'])?..];
	let level: i8 = value.split(char::is_alphabetic).nth(1)?.parse().ok()?;

	Some(if value.starts_with('s') { -level } else { level })
}

#[inline]
fn parse_levels (value: Vec<String>) -> Vec<i8> {
	let mut levels: Vec<_> = value.iter().map(|value| value.parse()).try_collect().unwrap_or_default();

	levels.sort_unstable();
	levels
}
