#![feature(option_result_contains)]


use gloo_events::EventListener;
use gloo_net::http::Request;
use leptos::*;
use orbit::components::{Viewer, ViewerProps};
use orbit::state::provide_state;
use web_sys::{KeyboardEvent, UrlSearchParams};

use viewer::Manifest;
use viewer::components::{Controls, ControlsProps, Sidebar, SidebarProps};


pub fn main () {
	mount_to_body(|scope| {
		let params = UrlSearchParams::new_with_str(&window().location().search().unwrap()).unwrap();
		let interactive = params.get("interactive").contains(&"true");
		let overlay = create_rw_signal(scope, false);
		let sidebar = create_rw_signal(scope, true);
		let url = create_rw_signal(scope, params.get("manifest"));

		let manifest = create_local_resource(scope, url, |url| async {
			let mut url = url?;

			if !(url.starts_with("http://") || url.starts_with("https://")) {
				url = document().base_uri().unwrap().unwrap() + &url;
			}

			let data = Request::get(&url)
				.send().await.unwrap()
				.binary().await.unwrap();

			#[cfg(debug_assertions)]
			web_sys::console::time_with_label("load manifest");

			let manifest: Manifest = serde_json::de::from_slice(&data).unwrap();

			#[cfg(debug_assertions)]
			web_sys::console::time_end_with_label("load manifest");

			Some(manifest)
		});

		let handler = EventListener::new(&document(), "keydown", move |event| {
			url.set(Some(String::from(match KeyboardEvent::code(event.unchecked_ref()).as_str() {
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
				let manifest = manifest.read()??;

				provide_state(scope, manifest.scene.into(), overlay.into());

				Some(view!(scope,
					<Viewer />

					{interactive.then(move || view!(scope,
						<section class="ui">
							<Controls overlay sidebar />

							<Sidebar project=&manifest.project visible=sidebar.into() />
						</section>
					))}
				))
			}}
		)
	});
}
