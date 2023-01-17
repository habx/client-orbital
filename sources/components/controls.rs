use std::rc::Rc;
use std::str::FromStr;

use leptos::*;
use orbit::state::use_state;

use crate::project::Project;


#[component]
pub fn Controls (
	scope: Scope,
	overlay: RwSignal<bool>,
	#[prop(into)]
	overlay_forced: Signal<bool>,
	project: Rc<Project>,
	sidebar: RwSignal<bool>,
) -> impl IntoView {
	let state = use_state(scope);

	view!(scope,
		<div class="controls">
			<button
				class="control"
				class:active=move || overlay.get() || overlay_forced.get()
				disabled=overlay_forced
				on:click=move |_| overlay.update(|overlay| *overlay = !*overlay)
			>
				"Toggle overlay"
			</button>

			<button
				class="control"
				on:click=move |_| sidebar.update(|sidebar| *sidebar = !*sidebar)
			>
				"Toggle sidebar"
			</button>

			<select
				class="control"
				on:change=move |event| if let Ok(camera) = usize::from_str(&event_target_value(&event)) {
					state.set_camera(camera);
				}
				prop:value=move || state.get_camera()
			>
				{project.cameras
					.iter()
					.enumerate()
					.map(|(index, camera)| view!(scope, <option value=index>{camera.label()}</option>))
					.collect::<Vec<_>>()}
			</select>
		</div>
	)
}
