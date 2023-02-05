#![cfg(target_arch = "wasm32")]


use std::time::Duration;

use futures::channel::mpsc::channel;
use futures::stream::StreamExt;
use leptos::*;
use wasm_bindgen_test::*;
use web_sys::{HtmlButtonElement, HtmlTitleElement};

use viewer::render;


wasm_bindgen_test_configure!(run_in_browser);


// TODO: Parallelize tests
#[wasm_bindgen_test]
async fn select_lot () {
	let (notify, mut result) = channel::<Result<(), String>>(1);

	mount_to_body(|scope| {
		const MANIFESTS: &[(&str, &[u8])] = &include!(concat!(env!("OUT_DIR"), "/manifests.rs"));


		let manifests = create_rw_signal(scope, MANIFESTS.to_vec());
		let errors = store_value(scope, Vec::new());

		create_effect(scope, move |_| {
			if let Some((project, _)) = manifests.with(|manifests| manifests.last().cloned()) {
				// TODO: Properly wait for the viewer to be ready
				set_timeout(move || {
					let lots = document().get_elements_by_class_name("card");

					for index in 0..lots.length() {
						let lot: HtmlButtonElement = lots.item(index).unwrap().unchecked_into();

						lot.click();

						if !lot.class_list().contains("active") {
							let identifier = lot
								.get_elements_by_class_name("card_title")
								.item(0).unwrap()
								.unchecked_into::<HtmlTitleElement>()
								.text_content().unwrap();

							errors.update(|errors| errors.push(format!("{project}: unable to select the lot {identifier}")));
						}
					}

					manifests.update(|manifests| {
						let _ = manifests.pop();
					});
				}, Duration::from_millis(200));
			} else {
				let _ = notify.clone().try_send(errors.with(|errors| if errors.is_empty() {
					Ok(())
				} else {
					Err(errors.join("\n"))
				}));
			}
		});

		view!(scope,
			// <style>
			// 	{include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sources/styles/base.css"))}
			// 	{include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sources/styles/wide.css"))}
			// </style>

			<div style="position:relative;width:0;height:0;overflow:hidden">
				{move || {
					let manifest = manifests.with(|manifests| serde_json::de::from_slice(&manifests.last().as_ref()?.1).ok())?;

					Some(render(scope, manifest, true, None, None))
				}}
			</div>
		)
	});

	result.next().await.unwrap().unwrap();
}
