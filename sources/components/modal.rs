use leptos::*;


#[component]
pub fn Modal (scope: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
	let open = create_rw_signal(scope, false);

	view!(scope,
		<button class="button" on:click=move |_| open.set(true)>
			"Aperçu 3D"
		</button>

		{move || open.get().then(|| view!(scope,
			<div class="modal">
				{children(scope)}

				<button class="control control-close" on:click=move |_| open.set(false)>
					<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
						<path d="m12 13.4-5.3 5.3-1.4-1.4 5.3-5.3-5.3-5.3 1.4-1.4 5.3 5.3 5.3-5.3 1.4 1.4-5.3 5.3 5.3 5.3-1.4 1.4z" />
					</svg>
				</button>
			</div>
		))}
	)
}
