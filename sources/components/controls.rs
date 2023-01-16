use leptos::*;


#[component]
pub fn Controls (scope: Scope, overlay: RwSignal<bool>, sidebar: RwSignal<bool>) -> impl IntoView {
	view!(scope,
		<div class="controls">
			<button
				class="control"
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
		</div>
	)
}
