use leptos::*;

use crate::lot::Role;
use crate::project::Project;


#[component]
pub fn Sidebar<'a> (scope: Scope, project: &'a Project) -> impl IntoView {
	let lots: Vec<_> = project.lots
			.iter()
			.filter(|lot| lot.role == Role::Living && lot.name.is_some())
			.collect();

	view!(scope,
		<aside class="sidebar">
			<h1>
				{if lots.len() > 1 {
					format!("{} lots", lots.len())
				} else {
					format!("{} lot", lots.len())
				}}
			</h1>

			{lots
				.iter()
				.map(|lot| view!(scope,
					<div>
						{lot.name.as_ref().unwrap()}
					</div>
				))
				.collect::<Vec<_>>()}
		</aside>
	)
}
