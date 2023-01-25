use crate::format;
use crate::project::Project;


#[derive(Clone, Debug)]
pub enum Camera {
	Level {
		absolute: i8,
		relative: u8,
	},
	Regular {
		label: String,
		name: String,
	}
}


impl Camera {
	#[inline]
	pub fn label (&self, project: &Project) -> String {
		match self {
			Self::Level { relative, .. } => format::level(project.absolute_level(*relative)),
			Self::Regular { label, .. } => label.clone(),
		}
	}
}
