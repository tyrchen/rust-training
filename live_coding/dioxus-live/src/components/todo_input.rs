use crate::Todos;
use dioxus::prelude::*;

pub fn todo_input(cx: Scope) -> Element {
    let todos = use_context::<Todos>(&cx)?;
    let draft = use_state(&cx, || "".to_string());

    rsx! {cx,
        header { class: "header",
            h1 { "todos" },
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                value: "{draft}",
                oninput: move |e| {
                    draft.set(e.value.clone());
                },
                onkeydown: move |e| {
                    if e.key == "Enter" && !draft.is_empty() {
                        todos.write().create_todo(draft.get());
                        draft.set("".to_string());
                    }
                }
            }
        }
    }
}
