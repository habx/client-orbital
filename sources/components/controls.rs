use leptos::*;


#[component]
pub fn Controls (scope: Scope, overlay: RwSignal<bool>) -> impl IntoView {
	view!(scope,
		<div class="controls">
			<button
				class="control"
				on:click=move |_| overlay.update(|overlay| *overlay = !*overlay)
			>
				"Toggle overlay"
			</button>
		</div>
	)
}
