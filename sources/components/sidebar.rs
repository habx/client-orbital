use std::rc::Rc;

use leptos::*;
use orbit::state::use_state;

use crate::camera::Camera;
use crate::lot::Role;
use crate::project::Project;


#[component]
pub fn Sidebar (
	scope: Scope,
	project: Rc<Project>,
	selected: RwSignal<Option<usize>>,
	#[prop(into)]
	visible: Signal<bool>,
) -> impl IntoView {
	let state = use_state(scope);
	let is_selected = create_selector(scope, selected);

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

						move |_| {
							let lot = &project.lots[index];

							selected.set(Some(index));

							let lot_level = project.relative_level(lot.level);
							let lot_camera = project.cameras
								.iter()
								.position(|camera| matches!(camera, Camera::Level(level) if *level == lot_level));

							if let Some(lot_camera) = lot_camera {
								state.set_camera(lot_camera);
							}
						}
					};

					view!(scope,
						<div on:click=set>
							{project.lots[index].name.clone().unwrap_or_default()}

							{move || is_selected(Some(index)).then(render_close_button)}
						</div>
					)
				})
				.collect::<Vec<_>>()}
		</aside>
	)
}
