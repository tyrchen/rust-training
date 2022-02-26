use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;

use crate::{TodoItem, Todos};

static NEXT_TODO_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Props)]
pub struct TodoInputProps<'a> {
    pub set_todos: &'a UseState<Todos>,
}

pub fn todo_input<'a>(cx: Scope<'a, TodoInputProps<'a>>) -> Element {
    let (draft, set_draft) = use_state(&cx, || "".to_string());
    let set_todos = cx.props.set_todos;

    rsx! {cx,
        header { class: "header",
            h1 { "todos" },
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                value: "{draft}",
                oninput: move |e| {
                    set_draft(e.value.clone());
                },
                onkeydown: move |e| {
                    if e.key == "Enter" && !draft.is_empty() {
                        let id = NEXT_TODO_ID.fetch_add(1, Ordering::Relaxed);
                        set_todos.make_mut().insert(id, TodoItem {
                            id,
                            title: draft.clone(),
                            completed: false,
                        });
                        set_draft("".to_string());
                    }
                }
            }
        }
    }
}
