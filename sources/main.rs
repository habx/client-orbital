use gloo_events::EventListener;
use gloo_net::http::Request;
use leptos::*;
use orbit::components::{Viewer, ViewerProps};
use web_sys::KeyboardEvent;

use viewer::Manifest;


pub fn main () {
	mount_to_body(|scope| {
		const MANIFEST: &str = "data/rueil-malmaison-l-imperiale.json";


		let manifest = create_rw_signal(scope, MANIFEST);

		let scene = create_local_resource(scope, manifest, |manifest| async {
			let data = Request::get(&(document().base_uri().unwrap().unwrap() + manifest))
				.send().await.unwrap()
				.binary().await.unwrap();

			#[cfg(debug_assertions)]
			web_sys::console::time_with_label("load scene");

			let scene = Manifest::from(serde_json::de::from_slice(&data).unwrap()).into();

			#[cfg(debug_assertions)]
			web_sys::console::time_end_with_label("load scene");
			scene
		});

		let handler = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
			"Digit1" => manifest.set(MANIFEST),
			"Digit2" => manifest.set("data/issy-les-moulineaux-joia.json"),
			"Digit3" => manifest.set("data/nantes-joneliere.json"),
			"Digit4" => manifest.set("data/le-plessis-robinson-agapanthe.json"),
			"Digit5" => manifest.set("data/bezannes-les-toits-du-golf.json"),
			"Digit6" => manifest.set("data/issy-les-moulineaux-carat.json"),
			_ => {}
		});

		on_cleanup(scope, move || drop(handler));

		view!(scope,
			{move || {
				let scene = scene.read()?;
					
				Some(view!(scope, <Viewer scene=scene />))
			}}
		)
	});
}
