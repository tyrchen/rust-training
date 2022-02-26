mod components;

use crate::components::{todo_filter, todo_input, todo_item};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use tracing::info;
use web_sys::Storage;

const TODO_KEY: &str = "todos_dioxus";
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todos {
    items: BTreeMap<u32, TodoItem>,
    next_id: u32,
}

impl Default for Todos {
    fn default() -> Self {
        let store = get_store();
        let default_todos = Todos {
            items: BTreeMap::new(),
            next_id: 1,
        };
        let todos = if let Ok(Some(todos)) = store.get(TODO_KEY) {
            serde_json::from_str(&todos).unwrap_or(default_todos)
        } else {
            default_todos
        };
        todos
    }
}

impl Deref for Todos {
    type Target = BTreeMap<u32, TodoItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Todos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl Todos {
    pub fn save(&self) {
        let store = get_store();
        let content = serde_json::to_string(self).unwrap();
        info!("saving todos: {content}");
        store.set(TODO_KEY, &content).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        let url_hash = web_sys::window().unwrap().location().hash().unwrap();
        match url_hash.as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
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

pub fn get_store() -> Storage {
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("User did not allow local storage")
}
