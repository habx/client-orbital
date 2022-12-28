use std::cell::Cell;
use std::rc::Rc;

use gloo_events::EventListener;
use leptos::*;
use orbit::components::{Viewer, ViewerProps};
use web_sys::KeyboardEvent;

use viewer::load_scene;


pub fn main () {
	mount_to_body(|scope| {
		let instance = Rc::new(Cell::new(None));
		let manifest = create_rw_signal(scope, 0);

		{
			let instance = instance.clone();

			let keydown = EventListener::new(&document(), "keydown", move |event| match KeyboardEvent::code(event.unchecked_ref()).as_str() {
				"Digit1" => manifest.set(0),
				"Digit2" => manifest.set(1),
				"Digit3" => manifest.set(2),
				"Digit4" => manifest.set(3),
				"Digit5" => manifest.set(4),
				"Digit6" => manifest.set(5),
				_ => {}
			});

			on_cleanup(scope, move || {
				drop(keydown);
				instance.take().map(ScopeDisposer::dispose);
			});
		}

		// FIXME: A wrapper is needed to circumvent the lack of fragments in `leptos@0.0.22`.
		view!(scope, 
			<div>
				{move || {
					const MANIFESTS: &[&str] = &[
						include_str!("../data/rueil-malmaison-l-imperiale.json"),
						include_str!("../data/issy-les-moulineaux-joia.json"),
						include_str!("../data/nantes-joneliere.json"),
						include_str!("../data/le-plessis-robinson-agapanthe.json"),
						include_str!("../data/bezannes-les-toits-du-golf.json"),
						include_str!("../data/issy-les-moulineaux-carat.json"),
					];


					#[cfg(debug_assertions)]
					web_sys::console::time();

					let scene = load_scene(MANIFESTS[manifest()]).unwrap();

					#[cfg(debug_assertions)]
					web_sys::console::time_end();

					let (element, disposer) = scope.run_child_scope(|scope| view!(scope, 
						<Viewer scene />
					));

					instance.replace(Some(disposer)).take().map(ScopeDisposer::dispose);
					element
				}}
			</div>
		)
	});
}
