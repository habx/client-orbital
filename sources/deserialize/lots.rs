use std::cell::RefCell;
use std::fmt;

use orbit::model::Shape;
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};

use crate::lot::Lot;

use super::geometry::GeometryVisitor;


struct LotVisitor<'a>(LotsVisitor<'a>);

#[derive(Clone, Copy)]
pub struct LotsVisitor<'a> {
	pub shapes: &'a RefCell<Vec<Shape>>,
}


impl<'de, 'a> DeserializeSeed<'de> for LotVisitor<'a> {
	type Value = Lot;


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
	type Value = Lot;


	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a `Lot` record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de> {
		let shapes = self.0.shapes;
		let start = shapes.borrow().len();
		let mut geometry = None;
		let mut identifier = None;
		let mut images = None;
		let mut name = None;
		let mut slug = None;
		let mut surface_area = None;
		let mut typology = None;

		while let Some(key) = map.next_key()? {
			match key {
				"geometry" => geometry = Some(map.next_value_seed(GeometryVisitor { shapes })?),
				"id" => identifier = map.next_value()?,
				"images" => images = map.next_value()?,
				"name" => name = map.next_value()?,
				"slug" => slug = map.next_value()?,
				"surfaceArea" => surface_area = map.next_value()?,
				"typology" => typology = map.next_value()?,
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		geometry.ok_or(Error::missing_field("geometry"))?;

		Lot::new(
			start..shapes.borrow().len(),
			identifier.ok_or(Error::missing_field("id"))?,
			images.unwrap_or_default(),
			name,
			slug,
			surface_area,
			typology,
		)
			.ok_or(Error::custom("`id` must follow BEL notation"))
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
			lots.push(lot);
		}

		Ok(lots)
	}
}
