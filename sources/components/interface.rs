use std::rc::Rc;

use leptos::*;
use orbit::state::use_state;
use web_sys::DomTokenList;

use crate::camera::Camera;
use crate::project::Project;

use super::controls::{Controls, ControlsProps};
use super::sidebar::{Sidebar, SidebarProps};


#[component]
pub fn Interface (
	scope: Scope,
	lot: RwSignal<Option<usize>>,
	overlay: RwSignal<bool>,
	project: Rc<Project>,
	#[prop(into)]
	selection: Signal<bool>,
) -> impl IntoView {
	let state = use_state(scope);
	let sidebar = create_rw_signal(scope, true);

	create_effect(scope, {
		let project = project.clone();

		move |previous: Option<Option<DomTokenList>>| {
			if !state.is_overlay_mounted() {
				return None
			}

			lot.with(|index| {
				if let Some(previous) = previous.flatten() {
					let _ = previous.remove_1("active").unwrap();
				}

				let current = document().get_element_by_id(&project.lots[(*index)?].identifier).unwrap().class_list();
				let _ = current.add_1("active").unwrap();

				Some(current)
			})
		}
	});

	// Resets the lot selection when switching camera
	create_effect(scope, {
		let project = project.clone();

		move |_| match &project.cameras[state.get_camera()] {
			Camera::Level(level) => project
				.lot_levels(lot.get_untracked()?, &state.get_scene().shapes)
				.all(|lot_level| lot_level != *level)
				.then(|| lot.set(None)),
			_ => None
		}
	});

	view!(scope,
		<section class="ui" class:selection=selection>
			<Controls
				overlay
				overlay_forced=selection
				project=project.clone()
				sidebar
			/>

			<Sidebar project selected=lot visible=sidebar />
		</section>
	)
}
