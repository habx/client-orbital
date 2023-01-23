use std::ops::Range;

use orbit::model::Shape;
use orbit::utils::dot_product;


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
	pub name: Option<String>,
	pub range: Range<usize>,
	pub role: Role,
	pub slug: Option<String>,
	pub surface_area: Option<u64>,
	pub typology: Option<u8>,
}


impl Lot {
	pub fn new (
		range: Range<usize>,
		identifier: String,
		images: Vec<(u8, String)>,
		name: Option<String>,
		slug: Option<String>,
		surface_area: Option<u64>,
		typology: Option<u8>,
	) -> Result<Self, String> {
		let value = identifier.to_lowercase();

		if let Some(((building, level), role)) = parse_building(&value).zip(parse_level(&value)).zip(Role::parse(&value)) {
			Ok(Self {
				building,
				floors: vec![range.start],
				identifier,
				images,
				level,
				name,
				range,
				role,
				slug,
				surface_area,
				typology,
			})
		} else {
			Err(identifier)
		}
	}

	pub fn class (&self) -> String {
		format!("lot{}", self.typology.map_or_else(String::new, |typology| format!(" t{}", typology)))
	}

	pub fn process (&mut self, shapes: &mut [Shape]) {
		let floors = &mut self.floors;

		if self.role == Role::Living {
			let start = self.range.start;
			let end = self.range.end;

			shapes[start..end].sort_unstable_by_key(|shape| (shape.center()[2] * 10_000.) as i64);

			floors.extend(shapes[start..end - 2]
				.windows(3)
				.enumerate()
				.filter_map(|(index, window)| (window[0].is_vertical() && !window[1].is_vertical() && !window[2].is_vertical())
					.then_some(start + index + 2)));

			for index in 0..floors.len() {
				let start = floors[index];
				let end = floors.get(index + 1).map_or(end - 1, |value| value - 1);

				// Floor
				let floor = &shapes[start];
				let group_center = floor.center();

				debug_assert!(!shapes[start].is_vertical());

				// Ceiling
				let ceiling = &mut shapes[end];

				debug_assert!(!ceiling.is_vertical());

				if ceiling.is_downward_facing() {
					ceiling.flip();
				}

				for shape in &mut shapes[start..end] {
					let flip = if shape.is_vertical() {
						dot_product(group_center - shape.center(), shape.normal()).is_sign_positive()
					} else {
						shape.is_upward_facing()
					};

					if flip {
						shape.flip();
					}
				}
			}
		}
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


fn parse_building (value: &str) -> Option<u8> {
	let value = &value[value.find('b')?..];

	Some(value.split(char::is_alphabetic).nth(1)?.parse::<u8>().ok()? - 1)
}

fn parse_level (value: &str) -> Option<i8> {
	let value = &value[value.find(['e', 's'])?..];
	let level: i8 = value.split(char::is_alphabetic).nth(1)?.parse().ok()?;

	Some(if value.starts_with('s') { -level } else { level })
}
