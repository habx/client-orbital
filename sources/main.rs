#![feature(option_result_contains)]


extern crate console_error_panic_hook;


use gloo_net::http::Request;
use leptos::{create_rw_signal, document, mount_to_body, spawn_local, window};
use web_sys::UrlSearchParams;

use viewer::render;


pub fn main () {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	mount_to_body(|scope| {
		let params = UrlSearchParams::new_with_str(&window().location().search().unwrap_or_default()).unwrap();

		debug_assert!(matches!(params.get("interactive").as_deref(), None | Some("true" | "false")));

		let redirection = params.get("redirection");

		debug_assert!(redirection
			.as_ref()
			.map(|redirection| redirection.starts_with("http://") || redirection.starts_with("https://"))
			.unwrap_or(true));

		let redirection_label = params.get("redirection_label");

		debug_assert!(redirection_label
			.as_ref()
			.map(|label| redirection.is_some() && !label.is_empty())
			.unwrap_or(true));

		let interactive = params.get("interactive").contains(&"true");
		let manifest = create_rw_signal(scope, None);

		if let Some(mut url) = params.get("manifest") {
			if !(url.starts_with("http://") || url.starts_with("https://")) {
				url = document().base_uri().unwrap().unwrap() + &url;
			}

			spawn_local(async move {
				let data = Request::get(&url)
					.send().await.unwrap()
					.binary().await.unwrap();

				#[cfg(debug_assertions)]
				web_sys::console::time_with_label("load manifest");

				manifest.set(Some(serde_json::de::from_slice(&data).unwrap()));

				#[cfg(debug_assertions)]
				web_sys::console::time_end_with_label("load manifest");
			});
		}

		move || Some(render(scope, manifest.get()?, interactive, redirection.clone(), redirection_label.clone()))
	});
}
