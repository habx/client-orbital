use leptos::*;
use orbit::state::use_viewer_state;

use crate::camera::Camera;
use crate::format;

use crate::context::use_context;

use super::images::{Images, ImagesProps};
use super::modal::{Modal, ModalProps};


#[component]
pub fn Sidebar (
	scope: Scope,
	redirection: Option<String>,
	redirection_label: Option<String>,
	selected: RwSignal<Option<usize>>,
	#[prop(into)]
	visible: Signal<bool>,
) -> impl IntoView {
	let project = use_context(scope);
	let viewer = use_viewer_state(scope);
	let is_selected = create_selector(scope, selected);

	let lots: Vec<_> = project.with(|project| (0..project.lots.len())
		.filter(|&index| project.lots[index].is_visible())
		.collect());

	view!(scope,
		<aside class="sidebar" class:open=visible>
			<div class="sidebar_content">
				<h1 class="sidebar_title">
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
						let active = Signal::derive(scope, move || is_selected(Some(index)));

						let toggle = move |_| if active() {
							selected.set(None)
						} else {
							selected.set(Some(index));
							project.with(|project| {
								let level = project.relative_level(project.lots[index].level);

								let camera = project.cameras
									.iter()
									.position(|camera| matches!(camera, Camera::Level { relative, .. } if *relative == level));

								if let Some(camera) = camera {
									viewer.set_camera(camera);
								}
							})
						};

						project.with(|project| {
							let lot = &project.lots[index];

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
					})
					.collect::<Vec<_>>()}
			</div>

			<footer class="sidebar_action">
				{move || {
					let index = selected.get()?;

					project
						.with(|project| !project.lots[index].images.is_empty())
						.then(|| view!(scope, <Modal><Images lot=index /></Modal>))
				}}

				{move || project.with(|project| selected
					.get()
					.zip(redirection.as_ref())
					.map(|(index, redirection)| {
						let lot = &project.lots[index];

						view!(scope,
							<a
								class="button"
								href=redirection
									.replace("%ID%", lot.slug.as_deref().unwrap_or_default())
									.replace("%SLUG%", lot.name.as_deref().unwrap_or_default())
								rel="noopener noreferrer"
								target="_blank"
							>
								{redirection_label.clone().unwrap_or(String::from("Voir les détails"))}
							</a>
						)
					}))}
			</footer>
		</aside>
	)
}
