mod components;
mod models;
mod platform;

pub(crate) use models::{Filter, Todos};

use crate::{
    components::{todo_filter, todo_input, todo_item},
    platform::{get_store, Store},
};
use dioxus::prelude::*;

use tracing::info;

pub fn app(cx: Scope) -> Element {
    // setup global context
    use_context_provider(&cx, || {
        let store = get_store();
        store.get()
    });
    use_context_provider(&cx, Filter::default);

    // consume todos / filter
    let todos = use_context::<Todos>(&cx)?;
    let filter = use_context::<Filter>(&cx)?;

    let filtered_todos = todos.read().get_filtered_todos(&filter.read());

    info!("filtered todos: {filtered_todos:?}");

    cx.render(rsx! {
        section { class: "todoapp",
            style { [include_str!("style.css")] },
            div {
                rsx!(todo_input())
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| {
                        rsx!(todo_item(key: "{id}", id: *id))
                    })
                }

                rsx!(todo_filter())
            }
        }
    })
}
