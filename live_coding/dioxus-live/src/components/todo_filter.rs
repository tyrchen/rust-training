use crate::{Filter, Todos};
use dioxus::prelude::*;

pub fn todo_filter(cx: Scope) -> Element {
    let todos = use_context::<Todos>(&cx)?;
    let todos_read = todos.read();

    let items_left = todos_read.items_left();
    let item_text = if items_left == 1 {
        "item left"
    } else {
        "items left"
    };

    let show_clear_completed = todos_read.show_clear_completed();

    rsx!(
        cx,
        (!todos_read.is_empty()).then(|| rsx! {
            footer { class: "footer",
                span { class: "todo-count",
                    strong { "{items_left}" },
                    span {" {item_text}" },
                }
                ul { class: "filters",
                    rsx!{cx, filter_item(item: Filter::All)}
                    rsx!{cx, filter_item(item: Filter::Active)}
                    rsx!{cx, filter_item(item: Filter::Completed)}
                }

                show_clear_completed.then(|| rsx! {
                    button {
                        class: "clear-completed",
                        onclick: move |_| {
                            todos.write().clear_completed();
                        },
                        "Clear completed",
                    }
                })
            }
        })
    )
}

#[inline_props]
fn filter_item(cx: Scope, item: Filter) -> Element {
    let filter = use_context::<Filter>(&cx)?;

    let class = if *filter.read() == *item {
        "selected"
    } else {
        ""
    };

    let onclick = move |_| *filter.write() = *item;

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
