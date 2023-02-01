#![feature(portable_simd)]


pub mod camera;
pub mod components;
pub mod context;
pub mod deserialize;
pub mod format;
pub mod lot;
pub mod project;


use leptos::*;
use orbit::components::{Viewer, ViewerProps};
use orbit::state::provide_viewer_state;

use self::components::{Interface, InterfaceProps};
use self::context::provide_context;

pub use self::deserialize::Manifest;


pub fn render (
	scope: Scope,
	manifest: Manifest,
	interactive: bool,
	redirection: Option<String>,
	redirection_label: Option<String>,
) -> impl IntoView {
	let lot = create_rw_signal(scope, None);
	let overlay = create_rw_signal(scope, false);
	let selection = create_memo(scope, move |_| lot.with(|lot| lot.is_some()));

	provide_context(scope, manifest.project);
	provide_viewer_state(scope, manifest.scene.into(), MaybeSignal::derive(scope, move || overlay.get() || selection.get()));

	view!(scope,
		<Interface
			interactive
			lot
			overlay
			redirection
			redirection_label
			selection
		/>

		<Viewer />
	)
}
