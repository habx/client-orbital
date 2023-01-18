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
				class:active=move || overlay.get() || overlay_forced.get()
				class="control control-overlay"
				disabled=overlay_forced
				on:click=move |_| overlay.update(|overlay| *overlay = !*overlay)
				title=move || if overlay.get() {
					"Masquer les calques de lots"
				} else {
					"Afficher les calques de lots"
				}
			>
				<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
					<path d="M23.9439 10.0001L12.0003 2.83386L0.0566406 10.0001L3.28864 11.9392L0.130859 14.0444L12.0004 21.1661L23.8699 14.0444L20.712 11.9392zM12.0003 14.8339L3.94394 10.0001L12.0003 5.16624L20.0566 10.0001z" />
				</svg>
			</button>

			<button
				class="control control-sidebar"
				on:click=move |_| sidebar.update(|sidebar| *sidebar = !*sidebar)
				title=move || if sidebar.get() { "Masquer le menu" } else { "Afficher le menu" }
			>
				<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
					<path d="M4 4H2V20H4V4ZM12.59 5.41L14 4L22 12L14 20L12.59 18.59L18.17 13H6V11H18.17z" />
				</svg>
			</button>

			<select
				class="control control-camera"
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
