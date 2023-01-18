use std::rc::Rc;

use leptos::*;
use orbit::state::use_state;

use crate::camera::Camera;
use crate::format;
use crate::lot::Role;
use crate::project::Project;


#[component]
pub fn Sidebar (
	scope: Scope,
	project: Rc<Project>,
	redirection: Option<String>,
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

	view!(scope,
		<aside class="sidebar" class:open=visible>
			<h1 class="sidebar_title">
				{if lots.len() > 1 {
					format!("{} lots", lots.len())
				} else {
					format!("{} lot", lots.len())
				}}
			</h1>

			<div class="sidebar_content">
				{lots
					.into_iter()
					.map(|index| {
						let is_selected = is_selected.clone();
						let active = Signal::derive(scope, move || is_selected(Some(index)));
						let lot = &project.lots[index];

						let toggle = {
							let project = project.clone();

							move |_| if active() {
								selected.set(None)
							} else {
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
							<button
								class="card"
								class:active=active
								on:click=toggle
								title=move || if active() {
									"Désélectionnner le lot"
								} else {
									"Afficher le lot"
								}
							>
								<div class="card_content">
									<h2 class="card_title">{lot.name.clone().unwrap_or_default()}</h2>

									{format::level(lot.level)}
								</div>

								<div>
									<div class="typology">{lot.typology.map(|typology| format!("T{typology}"))}</div>

									{lot.surface_area.map(|surface_area| format!("{:.1}m²", surface_area as f64 / 10_000.))}
								</div>
							</button>
						)
					})
					.collect::<Vec<_>>()}
			</div>

			{move || selected.get().zip(redirection.clone()).map(|(index, redirection)| {
				let lot = &project.lots[index];

				view!(scope,
					<footer class="sidebar_action">
						<a
							class="button"
							href=redirection
								.replace("%ID%", lot.slug.as_deref().unwrap_or_default())
								.replace("%SLUG%", lot.name.as_deref().unwrap_or_default())
							rel="noopener noreferrer"
							target="_blank"
						>
							"Voir le lot"
						</a>
					</footer>
				)
			})}
		</aside>
	)
}
