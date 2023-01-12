use std::cell::RefCell;
use std::fmt;

use orbit::model::Scene;
use serde::Deserialize;
use serde::de::{Error, IgnoredAny, MapAccess, Visitor};

use crate::project::Project;

use super::lots::LotsVisitor;
use super::meta::Meta;
use super::views::ViewsVisitor;


#[derive(Debug)]
pub struct Manifest(Scene);

struct ManifestVisitor;


impl<'de> Deserialize<'de> for Manifest {
	#[inline]
	fn deserialize<D> (deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		deserializer.deserialize_map(ManifestVisitor).map(Self)
	}
}

impl From<Manifest> for Scene {
	#[inline]
	fn from (manifest: Manifest) -> Self {
		manifest.0
	}
}

impl<'de> Visitor<'de> for ManifestVisitor {
	type Value = Scene;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de>, {
		let cameras = RefCell::new(Vec::new());
		let mut meta = None;
		let mut lots = None;
		let mut shapes = None;

		while let Some(key) = map.next_key()? {
			match key {
				"lots" => {
					let value = RefCell::new(Vec::new());

					lots = Some(map.next_value_seed(LotsVisitor { shapes: &value })?);
					shapes = Some(value);
				},
				"meta" => meta = Some(map.next_value()?),
				"views" => {
					let Meta { path, size } = meta.take().ok_or(Error::custom("field `meta` must precede `views`"))?;
					let lots = lots.take().ok_or(Error::custom("field `lots` must precede `views`"))?;
					let shapes = shapes.as_ref().unwrap();
					let project = Project::new(lots, &mut* shapes.borrow_mut()).unwrap();

					map.next_value_seed(ViewsVisitor {
						cameras: &cameras,
						height: size.height,
						identifier: None,
						path: &path,
						project: &project,
						shapes: &*shapes.borrow(),
						width: size.width,
					})?
				}
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		let mut cameras = cameras.into_inner();

		cameras[1..].reverse();
		Ok(Scene::new(cameras, shapes.unwrap().into_inner()))
	}
}