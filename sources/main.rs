#![feature(option_result_contains)]


use gloo_events::EventListener;
use gloo_net::http::Request;
use leptos::*;
use orbit::components::{Viewer, ViewerProps};
use web_sys::{KeyboardEvent, UrlSearchParams};

use viewer::Manifest;


pub fn main () {
	mount_to_body(|scope| {
		let params = UrlSearchParams::new_with_str(&window().location().search().unwrap()).unwrap();
		let interactive = params.get("interactive").contains(&"true");
		let manifest = create_rw_signal(scope, params.get("manifest"));

		let scene = create_local_resource(scope, manifest, |manifest| async {
			let mut manifest = manifest?;

			if !(manifest.starts_with("http://") || manifest.starts_with("https://")) {
				manifest = document().base_uri().unwrap().unwrap() + &manifest;
			}

			let data = Request::get(&manifest)
				.send().await.unwrap()
				.binary().await.unwrap();

			#[cfg(debug_assertions)]
			web_sys::console::time_with_label("load scene");

			let scene = Manifest::from(serde_json::de::from_slice(&data).unwrap()).into();

			#[cfg(debug_assertions)]
			web_sys::console::time_end_with_label("load scene");

			Some(scene)
		});

		let handler = EventListener::new(&document(), "keydown", move |event| {
			manifest.set(Some(String::from(match KeyboardEvent::code(event.unchecked_ref()).as_str() {
				"Digit1" => "data/rueil-malmaison-l-imperiale.json",
				"Digit2" => "data/issy-les-moulineaux-joia.json",
				"Digit3" => "data/nantes-joneliere.json",
				"Digit4" => "data/le-plessis-robinson-agapanthe.json",
				"Digit5" => "data/bezannes-les-toits-du-golf.json",
				"Digit6" => "data/issy-les-moulineaux-carat.json",
				_ => return
			})));
		});

		on_cleanup(scope, move || drop(handler));

		view!(scope,
			{move || {
				let scene = scene.read()??;
					
				Some(view!(scope, <Viewer scene=scene with_overlay=interactive />))
			}}
		)
	});
}
