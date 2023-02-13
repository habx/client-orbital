mod images;


use std::cell::RefCell;
use std::fmt;
use std::simd::{f64x4, Simd};

use orbit::model::{Camera, Shape, Style};
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};

use crate::camera::Camera as Identifier;
use crate::project::Project;

use self::images::ImagesVisitor;


struct ViewVisitor<'a>(u8, ViewsVisitor<'a>);

#[derive(Clone, Copy)]
pub struct ViewsVisitor<'a> {
	pub angles: &'a RefCell<Vec<RefCell<Vec<f64>>>>,
	pub cameras: &'a RefCell<Vec<Camera>>,
	pub height: usize,
	// FIXME: Come up with a better name
	pub identifiers: &'a RefCell<Vec<Identifier>>,
	pub path: &'a str,
	pub project: &'a Project,
	pub shapes: &'a [Shape],
	pub width: usize,
}


impl<'de, 'a> DeserializeSeed<'de> for ViewVisitor<'a> {
	type Value = ();


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_map(self)
	}
}

impl<'de, 'a> DeserializeSeed<'de> for ViewsVisitor<'a> {
	type Value = ();


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> Visitor<'de> for ViewVisitor<'a> {
	type Value = ();


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de> {
		let ViewsVisitor { cameras, height, identifiers, path, project, shapes, width, .. } = self.1;
		let initial_length = cameras.borrow().len();
		let mut identifier = None;
		let mut name = None;
		let mut viewports = None;

		while let Some(key) = map.next_key()? {
			match key {
				"floor" => identifier = Some(Identifier::Level {
					absolute: map.next_value()?,
					relative: self.0,
				}),
				"floors" => map.next_value_seed(self.1)?,
				"images" => {
					let angles = RefCell::new(Vec::new());

					viewports = Some(map.next_value_seed(ImagesVisitor { angles: &angles, path })?);
					self.1.angles.borrow_mut().push(angles);
				}
				"label" => identifier = Some(Identifier::Regular {
					label: map.next_value()?,
					name: name.take().unwrap(),
				}),
				"name" => name = map.next_value()?,
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		let identifier = identifier.ok_or(Error::missing_field("label or floor"))?;
		let mut viewports = viewports.ok_or(Error::missing_field("images"))?;
		let mut cameras = cameras.borrow_mut();
		let mut identifiers = identifiers.borrow_mut();
		let reverse = initial_length != cameras.len();

		if !matches!(&identifier, Identifier::Regular { name, .. } if name == "orbital_garden") {
			viewports.reverse();
		}

		let styles = match &identifier {
			Identifier::Level { relative, .. } => project.lots
				.iter()
				.filter_map(|lot| lot.is_visible().then(|| Style::compound(
					lot.identifier.clone(),
					lot.class(),
					lot.range
						.clone()
						.filter_map(|index| {
							const OFFSET: f64x4 = Simd::from_array([0., 0., 1.15, 0.]);


							let shape = &shapes[index];

							// Filters out walls, ceilings and other shapes not considered a part of the current level floor.
							(
								!shape.is_vertical() &&
								shape.is_height_negative() &&
								project.shape_relative_level(lot.building, shape) == *relative
							).then(|| {
								let is_indoor = lot.floors.contains(&index);

								Style::shape(format!("floor {}", if is_indoor { "in" } else { "out" }), index, false, is_indoor.then_some(OFFSET))
							})
						})
						.collect()
				)))
				.collect(),

			Identifier::Regular { .. } => project.lots
				.iter()
				.filter_map(|lot| lot.is_visible().then(|| Style::compound(
					lot.identifier.clone(),
					lot.class(),
					lot.range
						.clone()
						// Filters out floors and ceilings
						.filter_map(|index| (!shapes[index].is_horizontal()).then(|| Style::shape(format!("wall"), index, true, None)))
						.collect()
				)))
				.collect(),
		};

		cameras.push(Camera::new(width, height, viewports, styles));
		identifiers.push(identifier);

		if reverse {
			cameras.reverse();
			identifiers.reverse();
		}

		Ok(())
	}
}

impl<'de, 'a> Visitor<'de> for ViewsVisitor<'a> {
	type Value = ();


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an array of records")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		if let Some(capacity) = sequence.size_hint() {
			self.angles.borrow_mut().reserve(capacity);
			self.cameras.borrow_mut().reserve(capacity);
			self.identifiers.borrow_mut().reserve(capacity);
		}

		let mut level = 0;

		while sequence.next_element_seed(ViewVisitor(level, self))?.is_some() {
			level += 1;
		}

		Ok(())
	}
}
