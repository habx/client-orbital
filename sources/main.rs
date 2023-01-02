use std::cell::Cell;
use std::rc::Rc;

use gloo_events::EventListener;
use gloo_net::http::Request;
use leptos::*;
use orbit::components::{Viewer, ViewerProps};
use web_sys::KeyboardEvent;

use viewer::load_scene;


pub fn main () {
	mount_to_body(|scope| {
		const MANIFEST: &str = "data/rueil-malmaison-l-imperiale.json";


		let instance = Rc::new(Cell::new(None));
		let manifest = create_rw_signal(scope, MANIFEST);

		let scene = create_local_resource(scope, manifest, |manifest| async {
			let data = Request::get(&(document().base_uri().unwrap().unwrap() + manifest))
				.send().await.unwrap()
				.json().await.unwrap();

			#[cfg(debug_assertions)]
			web_sys::console::time_with_label("load scene");

			let scene = load_scene(data).unwrap();

			#[cfg(debug_assertions)]
			web_sys::console::time_end_with_label("load scene");
			scene
		});

		{
			let instance = instance.clone();

			let keydown = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
				"Digit1" => manifest.set(MANIFEST),
				"Digit2" => manifest.set("data/issy-les-moulineaux-joia.json"),
				"Digit3" => manifest.set("data/nantes-joneliere.json"),
				"Digit4" => manifest.set("data/le-plessis-robinson-agapanthe.json"),
				"Digit5" => manifest.set("data/bezannes-les-toits-du-golf.json"),
				"Digit6" => manifest.set("data/issy-les-moulineaux-carat.json"),
				_ => {}
			});

			on_cleanup(scope, move || {
				drop(keydown);
				instance.take().map(ScopeDisposer::dispose);
			});
		}

		let (element, disposer) = scope.run_child_scope(|scope| {
			// FIXME: A wrapper is needed to circumvent the lack of fragments in `leptos@0.0.22`.
			view!(scope, 
				<div>
					{move || {
						if let Some(scene) = scene.read() {
							vec![view!(scope, <Viewer scene=scene />)]
						} else {
							vec![]
						}
					}}
				</div>
			)
		});

		instance.replace(Some(disposer)).take().map(ScopeDisposer::dispose);
		element
	});
}
