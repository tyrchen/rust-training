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
    let (todos, set_todos) = use_state(&cx, || {
        let store = get_store();
        store.get()
    });
    let (filter, set_filter) = use_state(&cx, Filter::default);

    let filtered_todos = todos.get_filtered_todos(filter);

    info!("todos: {todos:?}, filtered todos: {filtered_todos:?}");

    cx.render(rsx! {
        section { class: "todoapp",
            style { [include_str!("style.css")] },
            div {
                rsx!(todo_input(set_todos: set_todos))
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| {
                        rsx!(todo_item(key: "{id}", id: *id, set_todos: set_todos))
                    })
                }

                rsx!(todo_filter(set_todos: set_todos, set_filter: set_filter))
            }
        }
    })
}
