use std::str::FromStr;

use leptos::*;
use orbit::state::use_viewer_state;

use crate::context::use_context;


#[component]
pub fn Controls (
	scope: Scope,
	interactive: bool,
	lot: RwSignal<Option<usize>>,
	overlay: RwSignal<bool>,
	selection: Signal<bool>,
	sidebar: RwSignal<bool>,
) -> impl IntoView {
	let project = use_context(scope);
	let viewer = use_viewer_state(scope);

	view!(scope,
		<div class="controls">
			{interactive.then(|| view!(scope,
				<button
					class:active=move || selection.get() || overlay.get()
					class="control control-overlay"
					on:click=move |_| if selection.get_untracked() {
						lot.set(None);
						overlay.set(false);
					} else {
						overlay.update(|overlay| *overlay = !*overlay);
					}
					title=move || if selection.get() {
						"Désélectionnner le lot"
					} else if overlay.get() {
						"Masquer les calques de lots"
					} else {
						"Afficher les calques de lots"
					}
				>
					<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
						<path d="M24 10 12 2.8 0 10l3.3 2L0 14l12 7.2L23.9 14l-3.2-2zm-12 4.8L4 10l8-4.8 8 4.8z" />
					</svg>
				</button>

				<button
					class="control control-sidebar"
					on:click=move |_| sidebar.update(|sidebar| *sidebar = !*sidebar)
					title=move || if sidebar.get() { "Masquer le menu" } else { "Afficher le menu" }
				>
					<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
						<path d="M4 4H2v16h2V4Zm8.6 1.4L14 4l8 8-8 8-1.4-1.4 5.6-5.6H6v-2h12.2z" />
					</svg>
				</button>
			))}

			<select
				class="control control-camera"
				on:change=move |event| if let Ok(camera) = usize::from_str(&event_target_value(&event)) {
					viewer.set_camera(camera);
				}
				on:keydown=|event| event.prevent_default()
				prop:value=move || viewer.get_camera()
			>
				{project.with(|project| project.cameras
					.iter()
					.enumerate()
					.map(|(index, camera)| view!(scope,
						<option value=index>{camera.label(&project)}</option>
					))
					.collect::<Vec<_>>())}
			</select>

			<svg
				class="compass"
				style=move || if let Some(angle) = project().angle(viewer.get_camera(), viewer.get_viewport()) {
					format!("transform:rotate({angle:.1}deg)")
				} else {
					format!("display:none")
				}
				viewBox="0 0 60 60"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path fill="#fff" fill-opacity=".6" d="M30 49.6a19.6 19.6 0 1 0 0-39.2 19.6 19.6 0 0 0 0 39.2Zm3.5-40.7h-2.8L29 6.8v2h-3.6V0h2.9l1.6 2V0h3.6v8.9Zm-2.4-3.5-3.4-4.2h-1.2v6.5H28V3.4l3.3 4.3h1.2V1.2H31v4.2Z" />

				<path d="M34.6 30 30 19.6 25.4 30l4.6 9.2 4.6-9.2Zm-7.3 0h5.4L30 35.4 27.3 30Zm-14.6 0a17.3 17.3 0 1 1 34.6 0 17.3 17.3 0 0 1-34.6 0ZM30 15a15 15 0 1 0 0 30 15 15 0 0 0 0-30Zm2.4-7.3V1.2H31v4.2l-3.4-4.2h-1.2v6.5H28V3.4l3.3 4.3h1.2Z" />
			</svg>
		</div>
	)
}
