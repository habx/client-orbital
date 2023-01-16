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
	pub level: i8,
	pub name: Option<String>,
	pub range: Range<usize>,
	pub role: Role,
	pub typology: Option<u8>,
}


impl Lot {
	pub fn new (
		range: Range<usize>,
		identifier: String,
		name: Option<String>,
		typology: Option<u8>,
	) -> Option<Self> {
		let value = identifier.to_lowercase();

		Some(Self {
			building: {
				let value = &value[value.find('b')?..];

				value.split(char::is_alphabetic).nth(1)?.parse().ok()?
			},
			floors: vec![range.start],
			identifier,
			level: {
				let value = &value[value.find(['e', 's'])?..];
				let level: i8 = value.split(char::is_alphabetic).nth(1)?.parse().ok()?;

				if value.starts_with('s') { -level } else { level }
			},
			name,
			range,
			role: Role::parse(&value)?,
			typology,
		})
	}

	pub fn class (&self) -> String {
		format!("lot{}", self.typology.map_or_else(String::new, |typology| format!(" t{}", typology)))
	}

	pub fn process (&mut self, shapes: &mut[Shape]) {
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

				if ceiling.normal()[2].is_sign_negative() {
					ceiling.flip();
				}

				for shape in &mut shapes[start..end] {
					let flip = if shape.is_vertical() {
						dot_product(group_center - shape.center(), shape.normal()).is_sign_positive()
					} else {
						shape.normal()[2].is_sign_positive()
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
