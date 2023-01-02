#![feature(drain_filter, portable_simd)]


use std::collections::BTreeSet;
use std::iter::once;
use std::mem::take;
use std::simd::{f64x4, Simd};

use orbit::model::{Camera, Scene, Shape, Source, Style, Viewport};
use orbit::utils::dot_product;
use serde::Deserialize;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
	Ceiling,
	Floor,
	Wall,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Level {
	Absolute(i8),
	Relative(i8, Option<i8>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
	Annex,
	Circulation,
	Living,
}


#[derive(Deserialize)]
#[serde(untagged)]
pub enum ViewId {
	Level(u8),
	Name(String),
}


#[derive(Deserialize)]
struct Face {
	v: Vec<usize>,
}

#[derive(Debug)]
struct Features {
	building: u8,
	kind: Kind,
	level: u8,
	role: Role,
	typology: u8,
}

#[derive(Deserialize)]
struct Floor {
	images: Vec<Image>,
}

#[derive(Deserialize)]
struct Geometry {
	vertices: Vec<[f64; 3]>,
	faces: Vec<Face>,
}

#[derive(Deserialize)]
struct Image {
	uri: String,
	m: [[f64; 4]; 4],
	// north: Option<f64>,
	p: [f64; 3],
}

#[derive(Deserialize)]
struct Lot {
	id: String,
	#[serde(default)]
	typology: u8,
	geometry: Geometry,
}

#[derive(Deserialize)]
pub struct Manifest {
	lots: Vec<Lot>,
	meta: Meta,
	views: Vec<View>,
}

#[derive(Deserialize)]
struct Meta {
	path: String,
	size: Size,
}

#[derive(Deserialize)]
struct Size {
	h: usize,
	w: usize,
}

#[derive(Deserialize)]
struct View {
	name: ViewId,
	#[serde(default)]
	floors: Vec<Floor>,
	images: Vec<Image>,
}


impl Role {
	fn parse (value: &str) -> Option<Self> {
		Some(match value[value.find(['c', 'h', 'l'])?..].chars().next()? {
			'c' => Self::Circulation,
			'h' => Self::Annex,
			'l' => Self::Living,
			_ => unreachable!()
		})
	}
}

pub fn load_scene (manifest: Manifest) -> Option<Scene> {
	let meta = manifest.meta;

	let (shapes, mut features): (Vec<_>, Vec<_>) = manifest.lots
		.iter()
		.flat_map(|lot| {
			let id = lot.id.to_lowercase();
			let building = parse_building(&id).unwrap();
			// let level = parse_level(&id).unwrap();
			let points: Vec<_> = lot.geometry.vertices.iter().map(convert_3).collect();
			let role = Role::parse(&id).unwrap();
			let typology = lot.typology.clamp(1, 6);

			let mut shapes: Vec<_> = lot.geometry.faces
				.iter()
				.map(move |face| {
					let shape = Shape::new(face.v.iter().map(|&index| points[index]).collect());
					let kind = if shape.is_vertical() { Kind::Wall } else { Kind::Floor };

					(shape, Features {
						building,
						kind,
						level: 0,
						role,
						typology,
					})
				})
				.collect();

			if role == Role::Living {
				shapes.sort_unstable_by_key(|(shape, ..)| (shape.center()[2] * 10_000.) as i64);

				once(0)
					.chain(shapes[..shapes.len() - 2]
						.windows(3)
						.enumerate()
						.filter_map(|(index, values)| (
							values[0].1.kind == Kind::Wall && 
							values[1].1.kind == Kind::Floor &&
							values[2].1.kind == Kind::Floor
						).then_some(index + 2)))
					.chain(once(shapes.len()))
					.collect::<Vec<_>>()
					.windows(2)
					.for_each(|window| {
						let (start, end) = (window[0], window[1] - 1);

						debug_assert_eq!(shapes[start].1.kind, Kind::Floor);

						let (ceiling, features_ceiling) = &mut shapes[end];

						debug_assert_eq!(features_ceiling.kind, Kind::Floor);
						features_ceiling.kind = Kind::Ceiling;

						if ceiling.normal()[2].is_sign_negative() {
							ceiling.flip();
						}

						let center = shapes[start].0.center();

						for (shape, features) in &mut shapes[start..end] {
							let need_flipping = match features.kind {
								Kind::Floor => shape.normal()[2].is_sign_positive(),
								_ => dot_product(center - shape.center(), shape.normal()).is_sign_positive(),
							};

							if need_flipping {
								shape.flip();
							}
						}
					});
			}

			shapes
		})
		.unzip();

	let mut walls: Vec<_> = (0..shapes.len())
		.filter(|index| match features[*index] {
			Features { kind, role, .. } => kind != Kind::Ceiling && role == Role::Living
		})
		.collect();

	let floors: Vec<_> = walls
		.drain_filter(|index| match features[*index] {
			Features { kind, .. } => kind == Kind::Floor
		})
		.collect();

	let levels: BTreeSet<_> = floors
		.iter()
		.map(|index| (shapes[*index].center()[2].round() as i64, features[*index].building))
		.collect();

	for index in 0..features.len() {
		let mut height = shapes[index].center()[2].round() as i64;
		let features = &mut features[index];

		if features.kind == Kind::Ceiling {
			height -= 1;
		}

		features.level = levels
			.iter()
			.filter(|value| value.1 == features.building)
			.position(|value| value.0 >= height)
			.unwrap_or_default() as _;
	}

	let cameras = manifest.views
		.into_iter()
		.rev()
		.flat_map(|mut view| take(&mut view.floors)
			.into_iter()
			.enumerate()
			.map(|(index, floor)| View { 
				floors: Vec::new(),
				images: floor.images,
				name: ViewId::Level(index as _),
			})
			.chain(once(view)))
		.map(|view| {
			let viewports = view.images
				.iter()
				.map(|image| Viewport {
					matrix: [
						convert_4(image.m[0]),
						convert_4(image.m[1]),
						convert_4(image.m[2]),
						convert_4(image.m[3]),
					],
					position: convert_3(&image.p),
					source: Some(Source::Dynamic(format!("https://cdn.habx.com/image/upload/c_scale,w_{{}}/v1/cdn/{}/{}", &meta.path, image.uri))),
				});

			Camera {
				aspect_ratio: format!("{}/{}", meta.size.w, meta.size.h),
				styles: match &view.name {
					ViewId::Level(level) => floors
						.iter()
						.filter_map(|&index| (features[index].level == *level).then(|| Style {
							index,
							name: format!("floor t{}", features[index].typology),
						}))
						.collect(),
					_ => walls
						.iter()
						.map(|&index| Style {
							index,
							name: format!("wall"),
						})
						.collect(),
				},
				viewports: match &view.name {
					ViewId::Name(name) if name == "orbital_garden" => viewports.collect(),
					_ => viewports.rev().collect()
				},
			}
		})
		.collect();

	Some(Scene { cameras, shapes })
}

fn parse_building (value: &str) -> Option<u8> {
	let value = &value[value.find('b')?..];
	
	value.split(char::is_alphabetic).nth(1)?.parse().ok()
}

fn parse_level (value: &str) -> Option<i8> {
	let value = &value[value.find(['e', 's'])?..];
	let level: i8 = value.split(char::is_alphabetic).nth(1)?.parse().ok()?;

	Some(if value.starts_with('s') { -level } else { level })
}


#[inline]
pub fn convert_3 (value: &[f64; 3]) -> f64x4 {
	convert_4([value[0], value[1], value[2], 0.])
}

#[inline]
pub fn convert_4 (value: [f64; 4]) -> f64x4 {
	Simd::from_array(value)
}