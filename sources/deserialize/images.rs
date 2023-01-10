use std::fmt;
use std::simd::Simd;

use orbit::model::Viewport;
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};

use super::points::PointVisitor;


struct ImageVisitor<'a>(ImagesVisitor<'a>);

#[derive(Clone, Copy)]
pub struct ImagesVisitor<'a> {
	pub path: &'a str,
}


impl<'de, 'a> DeserializeSeed<'de> for ImageVisitor<'a> {
	type Value = Viewport;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_map(self)
	}
}

impl<'de, 'a> DeserializeSeed<'de> for ImagesVisitor<'a> {
	type Value = Vec<Viewport>;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> Visitor<'de> for ImageVisitor<'a> {
	type Value = Viewport;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de> {
		let mut matrix = None;
		let mut position = None;
		let mut uri = None;

		while let Some(key) = map.next_key()? {
			match key {
				"m" => {
					let value = map.next_value::<[_; 4]>()?;

					matrix = Some([
						Simd::from_array(value[0]),
						Simd::from_array(value[1]),
						Simd::from_array(value[2]),
						Simd::from_array(value[3]),
					])
				},
				"p" => position = Some(map.next_value_seed(PointVisitor)?),
				"uri" => uri = Some(map.next_value::<String>()?),
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		let matrix = matrix.ok_or(Error::missing_field("m"))?;
		let position = position.ok_or(Error::missing_field("p"))?;
		let uri = uri.ok_or(Error::missing_field("uri"))?;
		let path = &self.0.path;

		Ok(Viewport::new(position, matrix, vec![
			(0, format!("https://cdn.habx.com/image/upload/v1/cdn/{}/{}", path, &uri)),
			(1_000, format!("https://cdn.habx.com/image/upload/c_scale,w_1000/v1/cdn/{}/{}", path, &uri)),
			(1_500, format!("https://cdn.habx.com/image/upload/c_scale,w_1500/v1/cdn/{}/{}", path, &uri)),
			(2_000, format!("https://cdn.habx.com/image/upload/c_scale,w_2000/v1/cdn/{}/{}", path, &uri)),
		]))
	}
}

impl<'de, 'a> Visitor<'de> for ImagesVisitor<'a> {
	type Value = Vec<Viewport>;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an array of record")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let mut viewports = if let Some(capacity) = sequence.size_hint() { Vec::with_capacity(capacity) } else { Vec::new() };

		while let Some(viewport) = sequence.next_element_seed(ImageVisitor(self))? {
			viewports.push(viewport);
		}

		Ok(viewports)
	}
}
