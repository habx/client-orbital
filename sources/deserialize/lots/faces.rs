use std::cell::RefCell;
use std::fmt;
use std::simd::f64x4;

use orbit::model::Shape;
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, IgnoredAny, MapAccess, SeqAccess, Visitor};


struct FaceVerticesVisitor<'a>(FaceVisitor<'a>);

#[derive(Clone, Copy)]
struct FaceVisitor<'a> {
	vertices: &'a [f64x4],
}

pub struct FacesVisitor<'a> {
	pub shapes: &'a RefCell<Vec<Shape>>,
	pub vertices: &'a [f64x4],
}


impl<'de, 'a> DeserializeSeed<'de> for FaceVerticesVisitor<'a> {
	type Value = Shape;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> DeserializeSeed<'de> for FaceVisitor<'a> {
	type Value = Shape;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_map(self)
	}
}

impl<'de, 'a> DeserializeSeed<'de> for FacesVisitor<'a> {
	type Value = ();


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> Visitor<'de> for FaceVerticesVisitor<'a> {
	type Value = Shape;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an array")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let mut vertices = if let Some(capacity) = sequence.size_hint() { Vec::with_capacity(capacity) } else { Vec::new() };

		while let Some(index) = sequence.next_element::<usize>()? {
			vertices.push(*self.0.vertices.get(index).ok_or(Error::custom("index not found"))?);
		}

		Ok(Shape::new(vertices))
	}
}

impl<'de, 'a> Visitor<'de> for FaceVisitor<'a> {
	type Value = Shape;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de> {
		let mut shape = None;

		while let Some(key) = map.next_key()? {
			match key {
				"v" => shape = Some(map.next_value_seed(FaceVerticesVisitor(self))?),
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		shape.ok_or(Error::missing_field("v"))
	}
}

impl<'de, 'a> Visitor<'de> for FacesVisitor<'a> {
	type Value = ();


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an array")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let mut shapes = self.shapes.borrow_mut();
		let vertices = &self.vertices;

		if let Some(capacity) = sequence.size_hint() {
			shapes.reserve(capacity);
		}

		while let Some(face) = sequence.next_element_seed(FaceVisitor { vertices })? {
			shapes.push(face);
		}

		Ok(())
	}
}
