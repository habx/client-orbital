#[derive(Clone, Debug)]
pub enum Camera {
	Level(u8),
	Regular {
		label: String,
		name: String,
	}
}


impl Camera {
	#[inline]
	pub fn label (&self) -> String {
		match self {
			// TODO: Convert to absolute levels
			Self::Level(level) => format!("Étage {level}"),
			Self::Regular { label, .. } => label.clone(),
		}
	}
}
