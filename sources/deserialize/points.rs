use std::fmt;
use std::simd::{f64x4, Simd};

use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, SeqAccess, Visitor};


pub struct PointVisitor;
pub struct PointsVisitor;


impl<'de> DeserializeSeed<'de> for PointVisitor {
	type Value = f64x4;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de> DeserializeSeed<'de> for PointsVisitor {
	type Value = Vec<f64x4>;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de> Visitor<'de> for PointVisitor {
	type Value = f64x4;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a 3D point")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let x = sequence.next_element()?.ok_or_else(|| Error::invalid_length(0, &"x coordinate"))?;
		let y = sequence.next_element()?.ok_or_else(|| Error::invalid_length(1, &"y coordinate"))?;
		let z = sequence.next_element()?.ok_or_else(|| Error::invalid_length(2, &"z coordinate"))?;

		if sequence.next_element::<f64>()?.is_none() {
			Ok(Simd::from_array([x, y, z, 0.]))
		} else {
			Err(Error::custom("too many coordinates"))
		}
	}
}

impl<'de> Visitor<'de> for PointsVisitor {
	type Value = Vec<f64x4>;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a 3D point")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let mut points = if let Some(capacity) = sequence.size_hint() { Vec::with_capacity(capacity) } else { Vec::new() };

		while let Some(point) = sequence.next_element_seed(PointVisitor)? {
			points.push(point);
		}

		Ok(points)
	}
}
