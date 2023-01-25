use leptos::*;
use orbit::state::use_state;
use web_sys::DomTokenList;

use crate::camera::Camera;

use crate::context::use_context;
use crate::lot::Lot;

use super::controls::{Controls, ControlsProps};
use super::sidebar::{Sidebar, SidebarProps};


#[component]
pub fn Interface (
	scope: Scope,
	interactive: bool,
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
	create_effect(scope, move |_| project.with(|project| {
		if let Camera::Level { relative, .. } = &project.cameras[state.get_camera()] {
			let Lot { building, floors, .. } = &project.lots[lot.get_untracked()?];
			let shapes = &state.get_scene().shapes;

			floors
				.iter()
				.all(|&index| project.shape_relative_level(*building, &shapes[index]) != *relative)
				.then(|| lot.set(None))
		} else {
			None
		}
	}));

	view!(scope,
		<section class="ui" class:selection=selection>
			{interactive.then(|| view!(scope,
				<Sidebar redirection selected=lot visible=sidebar />
			))}

			<Controls interactive lot overlay selection sidebar />
		</section>
	)
}
