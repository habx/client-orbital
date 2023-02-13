mod faces;
mod geometry;
mod images;


use std::cell::RefCell;
use std::fmt;

use orbit::model::Shape;
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};

use crate::lot::Lot;

use self::geometry::GeometryVisitor;
use self::images::ImagesVisitor;


struct LotVisitor<'a>(LotsVisitor<'a>);

#[derive(Clone, Copy)]
pub struct LotsVisitor<'a> {
	pub path: &'a str,
	pub shapes: &'a RefCell<Vec<Shape>>,
}


impl<'de, 'a> DeserializeSeed<'de> for LotVisitor<'a> {
	type Value = Option<Lot>;


	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_map(self)
	}
}

impl<'de, 'a> DeserializeSeed<'de> for LotsVisitor<'a> {
	type Value = Vec<Lot>;


	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> Visitor<'de> for LotVisitor<'a> {
	type Value = Option<Lot>;


	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a `Lot` record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de> {
		let LotsVisitor { path, shapes, .. } = self.0;

		let start = shapes.borrow().len();
		let mut geometry = None;
		let mut identifier = None;
		let mut images = Vec::new();
		let mut levels = Vec::new();
		let mut name = None;
		let mut slug = None;
		let mut surface_area = None;
		let mut typology = None;

		while let Some(key) = map.next_key()? {
			match key {
				"geometry" => geometry = Some(map.next_value_seed(GeometryVisitor { shapes })?),
				"id" => identifier = map.next_value()?,
				"images" => images = map.next_value_seed(ImagesVisitor { path })?,
				"levels" => levels = map.next_value()?,
				"name" => name = map.next_value()?,
				"slug" => slug = map.next_value()?,
				"surfaceArea" => surface_area = map.next_value()?,
				"typology" => typology = map.next_value()?,
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		let identifier = identifier.ok_or(Error::missing_field("id"))?;

		if geometry.is_none() {
			#[cfg(debug_assertions)]
			leptos::warn!("lot `{}` ignored: missing geometry", &identifier);

			return Ok(None)
		}

		let lot = Lot::new(
			start,
			&mut *shapes.borrow_mut(),
			identifier,
			images,
			levels,
			name,
			slug,
			surface_area,
			typology,
		);

		match lot {
			Ok(lot) => Ok(Some(lot)),
			Err(identifier) => {
				#[cfg(debug_assertions)]
				leptos::warn!("lot `{}` ignored: invalid identifier", &identifier);

				Ok(None)
			}
		}
	}
}

impl<'de, 'a> Visitor<'de> for LotsVisitor<'a> {
	type Value = Vec<Lot>;


	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an array")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let mut lots = if let Some(capacity) = sequence.size_hint() { Vec::with_capacity(capacity) } else { Vec::new() };

		while let Some(lot) = sequence.next_element_seed(LotVisitor(self))? {
			if let Some(lot) = lot {
				lots.push(lot);
			}
		}

		Ok(lots)
	}
}
