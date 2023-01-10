use serde::Deserialize;


#[derive(Deserialize)]
pub struct Meta {
	pub path: String,
	pub size: Size,
}

#[derive(Deserialize)]
pub struct Size {
	#[serde(rename = "h")]
	pub height: usize,
	#[serde(rename = "w")]
	pub width: usize,
}
