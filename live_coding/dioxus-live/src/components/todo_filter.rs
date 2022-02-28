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

    let show_clear_completed = todos.show_clear_completed();

    rsx!(
        cx,
        (!todos.is_empty()).then(|| rsx! {
            footer { class: "footer",
                span { class: "todo-count",
                    strong { "{items_left}" },
                    span {" {item_text}" },
                }
                ul { class: "filters",
                    rsx!{cx, filter_item(item: Filter::All, set_filter: set_filter)}
                    rsx!{cx, filter_item(item: Filter::Active, set_filter: set_filter)}
                    rsx!{cx, filter_item(item: Filter::Completed, set_filter: set_filter)}
                }

                show_clear_completed.then(|| rsx! {
                    button {
                        class: "clear-completed",
                        onclick: move |_| {
                            let mut todos = set_todos.make_mut();
                            todos.clear_completed();
                        },
                        "Clear completed",
                    }
                })
            }
        })
    )
}

#[derive(Props)]
struct FilterItemProps<'a> {
    pub item: Filter,
    pub set_filter: &'a UseState<Filter>,
}

fn filter_item<'a>(cx: Scope<'a, FilterItemProps<'a>>) -> Element {
    let item = cx.props.item;
    let set_filter = cx.props.set_filter;
    let filter = set_filter.get();

    let class = if filter.as_ref() == &item {
        "selected"
    } else {
        ""
    };

    let onclick = move |_| set_filter(item);

    #[cfg(feature = "web")]
    {
        let href = match item {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        };

        rsx! {cx,
            li {
                a { class: "{ class }", href: "{href}", onclick: onclick, "{item}" },
            }
        }
    }

    #[cfg(feature = "desktop")]
    rsx! {cx,
        li {
            a { class: "{ class }", onclick: onclick, "{item}" },
        }
    }
}
