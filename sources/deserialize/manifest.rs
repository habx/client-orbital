use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use orbit::model::Scene;
use serde::Deserialize;
use serde::de::{Error, IgnoredAny, MapAccess, Visitor};

use crate::project::Project;

use super::lots::LotsVisitor;
use super::meta::Meta;
use super::views::ViewsVisitor;


#[derive(Clone, Debug)]
pub struct Manifest {
	pub project: Rc<Project>,
	pub scene: Rc<Scene>,
}

struct ManifestVisitor;


impl<'de> Deserialize<'de> for Manifest {
	#[inline]
	fn deserialize<D> (deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		deserializer.deserialize_map(ManifestVisitor)
	}
}

impl<'de> Visitor<'de> for ManifestVisitor {
	type Value = Manifest;


	#[inline]
	fn expecting (&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a record")
	}

	fn visit_map<Map> (self, mut map: Map) -> Result<Self::Value, Map::Error> where Map: MapAccess<'de>, {
		let mut cameras = None;
		let mut lots = None;
		let mut meta = None;
		let mut project = None;
		let mut shapes = None;

		while let Some(key) = map.next_key()? {
			match key {
				"lots" => {
					let value = RefCell::new(Vec::new());

					lots = Some(map.next_value_seed(LotsVisitor { shapes: &value })?);
					shapes = Some(value);
				},
				"meta" => meta = map.next_value()?,
				"views" => {
					let Meta { path, size } = meta.take().ok_or(Error::custom("field `meta` must precede `views`"))?;
					let lots = lots.take().ok_or(Error::custom("field `lots` must precede `views`"))?;
					// TODO: Merge the different types into a single wrapper
					let angles = RefCell::new(Vec::new());
					let cameras_shared = RefCell::new(Vec::new());
					let identifiers = RefCell::new(Vec::new());
					let shapes = shapes.as_ref().unwrap();

					let mut value = Project::new(lots, &mut* shapes.borrow_mut()).unwrap();

					map.next_value_seed(ViewsVisitor {
						angles: &angles,
						cameras: &cameras_shared,
						height: size.height,
						identifiers: &identifiers,
						path: &path,
						project: &value,
						shapes: &*shapes.borrow(),
						width: size.width,
					})?;

					let mut angles = angles.into_inner();
					let mut cameras_shared = cameras_shared.into_inner();
					let mut identifiers = identifiers.into_inner();

					if !angles.is_empty() {
						angles[1..].reverse();
						value.set_angles(angles);
					}

					cameras_shared[1..].reverse();
					identifiers[1..].reverse();
					value.set_cameras(identifiers);
					cameras = Some(cameras_shared);
					project = Some(value);
				}
				_ => { map.next_value::<IgnoredAny>()?; }
			}
		}

		Ok(Manifest {
			project: Rc::new(project.unwrap()),
			scene: Rc::new(Scene::new(cameras.unwrap(), shapes.unwrap().into_inner())),
		})
	}
}
