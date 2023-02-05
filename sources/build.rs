use std::env::var;
use std::error::Error;
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::Path;


fn main() -> Result<(), Box<dyn Error>> {
	let source = concat!(env!("CARGO_MANIFEST_DIR"), "/data");
	let mut manifests = File::create(&Path::new(&var("OUT_DIR")?).join("manifests.rs"))?;

	println!("cargo:rerun-if-changed={source}");
	writeln!(&mut manifests, r##"["##,)?;

	for file in read_dir(source)? {
		let file = file?;

		if file.file_type()?.is_file() {
			let file_name = file.file_name().into_string().unwrap();
			let project = file_name.strip_suffix(".json").unwrap();

			writeln!(&mut manifests, r##"("{project}", include_bytes!(r#"{}"#)),"##, file.path().display())?;
		}
	}

	writeln!(&mut manifests, r##"]"##,)?;
	Ok(())
}
