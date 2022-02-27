mod components;
mod platform;

use crate::{
    components::{todo_filter, todo_input, todo_item},
    platform::{get_store, Store},
};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use tracing::info;
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
        Self {
            items: BTreeMap::new(),
            next_id: 1,
        }
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
        info!("saving todos: {self:?}");
        store.set(self);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

pub fn app(cx: Scope) -> Element {
    let (todos, set_todos) = use_state(&cx, || {
        let store = get_store();
        store.get()
    });
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
                        rsx!(todo_item(key: "{id}", id: *id, set_todos: set_todos))
                    })
                }

                rsx!(todo_filter(set_todos: set_todos, set_filter: set_filter))
            }
        }
    })
}
