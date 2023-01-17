use std::cell::RefCell;
use std::fmt;
use std::simd::{f64x4, Simd};

use orbit::model::{Camera, Shape, Style};
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};

use crate::camera::Camera as Identifier;
use crate::project::Project;
use crate::lot::Role;

use super::images::ImagesVisitor;


struct ViewVisitor<'a>(ViewsVisitor<'a>);

pub struct ViewsVisitor<'a> {
	pub cameras: &'a RefCell<Vec<Camera>>,
	pub height: usize,
	pub identifier: Option<Identifier>,
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
		let ViewsVisitor { cameras, height, mut identifier, identifiers, path, project, shapes, width, .. } = self.0;
		let initial_length = cameras.borrow().len();
		let mut viewports = None;
		let mut name = None;

		while let Some(key) = map.next_key()? {
			match key {
				"floors" => map.next_value_seed(ViewsVisitor { identifier: None, ..self.0 })?,
				"images" => viewports = Some(map.next_value_seed(ImagesVisitor { path })?),
				"label" => identifier = Some(Identifier::Regular {
					label: map.next_value()?,
					name: name.take().unwrap(),
				}),
				"name" => name = Some(map.next_value()?),
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		let identifier = identifier.ok_or(Error::missing_field("name"))?;
		let mut viewports = viewports.ok_or(Error::missing_field("viewports"))?;
		let mut cameras = cameras.borrow_mut();
		let mut identifiers = identifiers.borrow_mut();
		let reverse = initial_length != cameras.len();

		if !matches!(&identifier, Identifier::Regular { name, .. } if name == "orbital_garden") {
			viewports.reverse();
		}

		let styles = match &identifier {
			Identifier::Level(level) => project.lots
				.iter()
				.filter_map(|lot| (lot.role == Role::Living && lot.name.is_some()).then(|| Style::compound(
					lot.identifier.clone(),
					lot.class(),
					lot.range
						.clone()
						.filter_map(|index| {
							const OFFSET: f64x4 = Simd::from_array([0., 0., 1.15, 0.]);


							let shape = &shapes[index];

							(
								!shape.is_vertical() &&
								shape.normal()[2].is_sign_negative() &&
								project.level(lot.building, shape) == *level
							).then(|| Style::shape(format!("floor"), index, lot.floors.contains(&index).then_some(OFFSET)))
						})
						.collect()
				)))
				.collect(),

			Identifier::Regular { .. } => project.lots
				.iter()
				.filter_map(|lot| (lot.role == Role::Living && lot.name.is_some()).then(|| Style::compound(
					lot.identifier.clone(),
					lot.class(),
					lot.range
						.clone()
						.filter_map(|index| shapes[index].is_vertical().then(|| Style::shape(format!("wall"), index, None)))
						.collect()
				)))
				.collect(),
		};

		cameras.push(Camera::new(format!("{width}/{height}"), viewports, styles));
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
			self.cameras.borrow_mut().reserve(capacity);
			self.identifiers.borrow_mut().reserve(capacity);
		}

		let mut level = 0;

		while sequence.next_element_seed(ViewVisitor(Self { identifier: Some(Identifier::Level(level)), ..self }))?.is_some() {
			level += 1;
		}

		Ok(())
	}
}
