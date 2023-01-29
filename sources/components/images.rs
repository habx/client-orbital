use leptos::*;

use crate::context::use_context;


#[component]
pub fn Arrow (scope: Scope) -> impl IntoView {
	view!(scope,
		<svg viewBox="0 0 10 16" xmlns="http://www.w3.org/2000/svg">
			<path d="M1.7.3.3 1.7 6.6 8 .3 14.3l1.4 1.4L9.4 8z" />
		</svg>
	)
}

#[component]
pub fn Images (scope: Scope, lot: usize) -> impl IntoView {
	let project = use_context(scope);
	let index = create_rw_signal(scope, 0);

	view!(scope,
		<figure class="image">
			<img
				alt=""
				src=move || project.with(|project| project.lots[lot].images[index.get()].1.clone())
			/>

			// TODO: Add a caption
			<figcaption>{}</figcaption>
		</figure>

		<button
			class="control control-previous"
			on:click=move |_| index.update(|index| {
				if *index == 0 {
					*index = project.with(|project| project.lots[lot].images.len()) - 1;
				} else {
					*index -= 1;
				}
			})
		>
			<Arrow />
		</button>

		<button
			class="control control-next"
			on:click=move |_| index.update(|index| {
				if *index == project.with(|project| project.lots[lot].images.len()) - 1 {
					*index = 0;
				} else {
					*index += 1
				}
			})
		>
			<Arrow />
		</button>
	)
}


