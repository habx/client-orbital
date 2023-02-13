mod lots;
mod manifest;
mod meta;
mod points;
mod views;


pub use self::manifest::Manifest;


#[cfg(test)]
mod tests {
	use std::error::Error;
	use std::fs::read;
	use std::path::Path;

	use super::Manifest;


	#[test]
	fn deserialize () -> Result<(), Box<dyn Error>> {
		for entry in Path::new(&format!("{}/data", env!("CARGO_MANIFEST_DIR"))).read_dir()? {
			let path = entry?.path();

			eprintln!("> {:?}", path);

			if let Err(error) = serde_json::de::from_slice::<Manifest>(&read(&path)?) {
				eprintln!("  {:?} {:?}", path, error);
			}
		}

		Ok(())
	}
}
