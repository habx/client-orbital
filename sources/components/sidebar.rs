use std::rc::Rc;

use leptos::*;
use orbit::state::use_state;

use crate::lot::Role;
use crate::project::Project;


#[component]
pub fn Sidebar (
	scope: Scope,
	project: Rc<Project>,
	selected: RwSignal<Option<String>>,
	#[prop(into)]
	visible: Signal<bool>,
) -> impl IntoView {
	let state = use_state(scope);

	let is_selected = {
		let project = project.clone();

		create_selector(scope, move || selected.get()
			.and_then(|selected| project.lots
				.iter()
				.position(|lot| lot.identifier == selected))
			.unwrap_or(project.lots.len()))
	};

	let lots: Vec<_> = (0..project.lots.len())
		.filter(|&index| {
			let lot = &project.lots[index];

			lot.role == Role::Living && lot.name.is_some()
		})
		.collect();

	let render_close_button = move || view!(scope,
		<button on:click=move |_| selected.set(None)>"Close"</button>
	);

	view!(scope,
		<aside class="sidebar" class:open=visible>
			<h1>
				{if lots.len() > 1 {
					format!("{} lots", lots.len())
				} else {
					format!("{} lot", lots.len())
				}}
			</h1>

			{lots
				.into_iter()
				.map(|index| {
					let is_selected = is_selected.clone();

					let set = {
						let project = project.clone();

						move |_| selected.set(Some(project.lots[index].identifier.clone()))
					};

					view!(scope,
						<div on:click=set>
							{project.lots[index].name.clone().unwrap_or_default()}

							{move || is_selected(index).then(render_close_button)}
						</div>
					)
				})
				.collect::<Vec<_>>()}
		</aside>
	)
}
