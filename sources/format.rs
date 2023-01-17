pub fn level (level: i8) -> String {
	if level == 0 {
		return "Rez-de-chaussée".into()
	}

	let label = if level.is_positive() { "étage" } else { "sous-sol" };

	match level.abs() {
		1 => format!("1er {label}"),
		level => format!("{level}ème {label}"),
	}
}
