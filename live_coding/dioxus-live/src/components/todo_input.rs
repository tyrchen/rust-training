use crate::Todos;
use dioxus::prelude::*;

pub fn todo_input(cx: Scope) -> Element {
    let todos = use_context::<Todos>(&cx)?;
    let (draft, set_draft) = use_state(&cx, || "".to_string());

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
                        todos.write().create_todo(draft);
                        set_draft("".to_string());
                    }
                }
            }
        }
    }
}
