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
						<path d="M12.0001 13.4142L6.70718 18.7071L5.29297 17.2928L10.5859 12L5.29297 6.70706L6.70718 5.29285L12.0001 10.5857L17.293 5.29285L18.7072 6.70706L13.4143 12L18.7072 17.2928L17.293 18.7071z" />
					</svg>
				</button>
			</div>
		))}
	)
}
