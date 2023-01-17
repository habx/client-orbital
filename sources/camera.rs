use crate::format;
use crate::project::Project;


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
	pub fn label (&self, project: &Project) -> String {
		match self {
			Self::Level(level) => format::level(project.absolute_level(*level)),
			Self::Regular { label, .. } => label.clone(),
		}
	}
}
