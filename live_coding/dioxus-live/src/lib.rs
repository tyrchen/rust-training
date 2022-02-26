mod components;

use crate::components::{todo_filter, todo_input, todo_item};
use dioxus::prelude::*;
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub type Todos = BTreeMap<u32, TodoItem>;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

pub fn app(cx: Scope) -> Element {
    let (todos, set_todos) = use_state(&cx, Todos::default);
    let (filter, set_filter) = use_state(&cx, Filter::default);

    let filtered_todos = todos
        .iter()
        .filter(|(_, todo)| match filter {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        })
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();

    info!("todos: {todos:?}, filtered todos: {filtered_todos:?}");

    cx.render(rsx! {
        section { class: "todoapp",
            style { [include_str!("style.css")] },
            div {
                rsx!(todo_input(set_todos: set_todos))
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| {
                        rsx!(todo_item(id: *id, set_todos: set_todos))
                    })
                }

                rsx!(todo_filter(set_todos: set_todos, set_filter: set_filter))
            }
        }
    })
}
