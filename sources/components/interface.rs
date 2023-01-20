use leptos::*;
use orbit::state::use_state;
use web_sys::DomTokenList;

use crate::camera::Camera;

use crate::context::use_context;

use super::controls::{Controls, ControlsProps};
use super::sidebar::{Sidebar, SidebarProps};


#[component]
pub fn Interface (
	scope: Scope,
	lot: RwSignal<Option<usize>>,
	overlay: RwSignal<bool>,
	redirection: Option<String>,
	#[prop(into)]
	selection: Signal<bool>,
) -> impl IntoView {
	let project = use_context(scope);
	let state = use_state(scope);
	let sidebar = create_rw_signal(scope, true);

	create_effect(scope, move |previous: Option<Option<DomTokenList>>| {
		if !state.is_overlay_mounted() {
			return None
		}

		lot.with(|index| {
			if let Some(previous) = previous.flatten() {
				let _ = previous.remove_1("active");
			}

			let index = *index.as_ref()?;
			let current = project.with(|project| document().get_element_by_id(&project.lots[index].identifier).unwrap()).class_list();
			let _ = current.add_1("active");

			Some(current)
		})
	});

	// Resets the lot selection when switching camera
	create_effect(scope, move |_| project.with(|project| match &project.cameras[state.get_camera()] {
		Camera::Level(level) => project
			.lot_levels(lot.get_untracked()?, &state.get_scene().shapes)
			.all(|lot_level| lot_level != *level)
			.then(|| lot.set(None)),
		_ => None
	}));

	view!(scope,
		<section class="ui" class:selection=selection>
			<Sidebar redirection selected=lot visible=sidebar />

			<Controls lot overlay selection sidebar />
		</section>
	)
}
