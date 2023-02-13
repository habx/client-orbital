use std::cell::RefCell;
use std::fmt;

use orbit::model::Shape;
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, Visitor};

use crate::deserialize::points::PointsVisitor;

use super::faces::FacesVisitor;


pub struct GeometryVisitor<'a> {
	pub shapes: &'a RefCell<Vec<Shape>>,
}


impl<'de, 'a> DeserializeSeed<'de> for GeometryVisitor<'a> {
	type Value = ();


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_map(self)
	}
}

impl<'de, 'a> Visitor<'de> for GeometryVisitor<'a> {
	type Value = ();


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de> {
		let shapes = self.shapes;
		let mut vertices = None;

		while let Some(key) = map.next_key()? {
			match key {
				"vertices" => vertices = Some(map.next_value_seed(PointsVisitor)?),
				"faces" => {
					let vertices = &vertices.take().ok_or(Error::custom("field `vertices` must precede `faces`"))?;

					map.next_value_seed(FacesVisitor { shapes, vertices })?;
				}
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		Ok(())
	}
}
