use std::rc::Rc;

use leptos::{store_value, Scope, StoredValue};

use super::project::Project;


// TODO: Create an actual useful context
#[derive(Clone, Copy)]
pub struct Context {
	pub project: StoredValue<Rc<Project>>,
}


pub fn provide_context (scope: Scope, project: Rc<Project>) {
	leptos::provide_context(scope, Context {
		project: store_value(scope, project),
	});
}

pub fn use_context (scope: Scope) -> StoredValue<Rc<Project>> {
	leptos::use_context::<Context>(scope).unwrap().project
}
