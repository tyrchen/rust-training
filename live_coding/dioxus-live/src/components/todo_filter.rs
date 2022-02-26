use crate::{Filter, Todos};
use dioxus::prelude::*;

#[derive(Props)]
pub struct TodoFilterProps<'a> {
    pub set_todos: &'a UseState<Todos>,
    pub set_filter: &'a UseState<Filter>,
}

pub fn todo_filter<'a>(cx: Scope<'a, TodoFilterProps<'a>>) -> Element {
    let set_todos = cx.props.set_todos;
    let set_filter = cx.props.set_filter;
    let filter = set_filter.get();
    let todos = set_todos.get();

    let items_left = todos.iter().fold(
        0,
        |acc, (_, todo)| if todo.completed { acc } else { acc + 1 },
    );
    let item_text = if items_left == 1 {
        "item left"
    } else {
        "items left"
    };

    let show_clear_completed = todos.iter().any(|(_, todo)| todo.completed);
    let active_text = |f| {
        if filter.as_ref() == &f {
            "selected"
        } else {
            ""
        }
    };
    let all_class = active_text(Filter::All);
    let active_class = active_text(Filter::Active);
    let completed_class = active_text(Filter::Completed);

    rsx!(
        cx,
        (!todos.is_empty()).then(|| rsx! {
            footer { class: "footer",
                span { class: "todo-count",
                    strong { "{items_left}" },
                    span {" {item_text}" },
                }
                ul { class: "filters",
                    li {
                        a { class: "{ all_class }", href: "#/", onclick: move |_| set_filter(Filter::All),  "All" },
                    },
                    li {
                    a { class: "{ active_class }", href: "#/active", onclick: move |_| set_filter(Filter::Active),  "Active" },
                    },
                    li {
                    a { class: "{ completed_class }", href: "#/completed", onclick: move |_| set_filter(Filter::Completed),  "Completed" },

                    },
                }

                show_clear_completed.then(|| rsx! {
                    button {
                        class: "clear-completed",
                        onclick: move |_| {
                            let mut todos = set_todos.make_mut();
                            todos.retain(|_, todo| !todo.completed);
                        },
                        "Clear completed",
                    }
                })
            }
        })
    )
}
