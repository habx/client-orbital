use std::fmt;

use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, SeqAccess, Visitor};


struct ImageVisitor<'a>(ImagesVisitor<'a>);

#[derive(Clone, Copy)]
pub struct ImagesVisitor<'a> {
	pub path: &'a str,
}


impl<'de, 'a> DeserializeSeed<'de> for ImageVisitor<'a> {
	type Value = (u8, String);


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> DeserializeSeed<'de> for ImagesVisitor<'a> {
	type Value = Vec<(u8, String)>;


	#[inline]
	fn deserialize<D> (self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_seq(self)
	}
}

impl<'de, 'a> Visitor<'de> for ImageVisitor<'a> {
	type Value = (u8, String);


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a tuple")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let level = sequence.next_element()?.ok_or(Error::missing_field("level"))?;
		let uri = sequence.next_element::<&str>()?.ok_or(Error::missing_field("uri"))?;

		Ok((level, format!("{}/dollhouses/{uri}", &self.0.path)))
	}
}

impl<'de, 'a> Visitor<'de> for ImagesVisitor<'a> {
	type Value = Vec<(u8, String)>;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("an array")
	}

	fn visit_seq<Sequence> (self, mut sequence: Sequence) -> Result<Self::Value, Sequence::Error> where Sequence: SeqAccess<'de> {
		let mut images = if let Some(capacity) = sequence.size_hint() { Vec::with_capacity(capacity) } else { Vec::new() };

		while let Some(image) = sequence.next_element_seed(ImageVisitor(self))? {
			images.push(image);
		}

		Ok(images)
	}
}
