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
    // setup global context
    use_context_provider(&cx, || {
        let store = get_store();
        store.get()
    });
    use_context_provider(&cx, Filter::default);

    // consume todos / filter
    let todos = use_context::<Todos>(&cx)?;
    let filter = use_context::<Filter>(&cx)?;

    let filtered_todos = todos.read().get_filtered_todos(&filter.read());

    info!("filtered todos: {filtered_todos:?}");

    cx.render(rsx! {
        navbar()
        section { class: "todoapp",
            style { [include_str!("style.css")] },
            div {
                rsx!(todo_input())
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| {
                        rsx!(todo_item(key: "{id}", id: *id))
                    })
                }

                rsx!(todo_filter())
            }
        }
    })
}

pub fn navbar(cx: Scope) -> Element {
    rsx! {cx,
        nav {
            class: "bg-gray-800",
            div {
                class: "max-w-7xl mx-auto px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex items-center justify-between h-16",
                    div {
                        class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button {
                            class: "inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white",
                            "aria-controls": "mobile-menu",
                            "aria-expanded": "false",
                            r#type: "button",span {
                                class: "sr-only",
                                "Open main menu"
                            }
                        }
                    }
                    div {
                        class: "flex-1 flex items-center justify-center sm:items-stretch sm:justify-start",
                        div {
                            class: "flex-shrink-0 flex items-center",
                            img {
                                class: "block lg:hidden h-8 w-auto",
                                alt: "Workflow",
                                src: "https://tailwindui.com/img/logos/workflow-mark-indigo-500.svg",
                            }
                            img {
                                class: "hidden lg:block h-8 w-auto",
                                alt: "Workflow",
                                src: "https://tailwindui.com/img/logos/workflow-logo-indigo-500-mark-white-text.svg",
                            }
                        }
                        div {
                            class: "hidden sm:block sm:ml-6",
                            div {
                                class: "flex space-x-4",
                                a {
                                    class: "bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium",
                                    "aria-current": "page",
                                    href: "#","Dashboard"
                                }
                                a {
                                    class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                                    href: "#","Team"
                                }
                                a {
                                    class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                                    href: "#","Projects"
                                }
                                a {
                                    class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                                    href: "#","Calendar"
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                        button {
                            class: "bg-gray-800 p-1 rounded-full text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white",
                            r#type: "button",span {
                                class: "sr-only",
                                "View notifications"
                            }
                        }
                        div {
                            class: "ml-3 relative",
                            div {
                                button {
                                    class: "bg-gray-800 flex text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white",
                                    id: "user-menu-button",
                                    "aria-expanded": "false",
                                    "aria-haspopup": "true",
                                    r#type: "button",span {
                                        class: "sr-only",
                                        "Open user menu"
                                    }
                                    img {
                                        class: "h-8 w-8 rounded-full",
                                        alt: "",
                                        src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
                                    }
                                }
                            }
                            div {
                                class: "origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none",
                                "aria-labelledby": "user-menu-button",
                                "aria-orientation": "vertical",
                                role: "menu",
                                tabindex: "-1",a {
                                    class: "block px-4 py-2 text-sm text-gray-700",
                                    id: "user-menu-item-0",
                                    href: "#",
                                    role: "menuitem",
                                    tabindex: "-1","Your Profile"
                                }
                                a {
                                    class: "block px-4 py-2 text-sm text-gray-700",
                                    id: "user-menu-item-1",
                                    href: "#",
                                    role: "menuitem",
                                    tabindex: "-1","Settings"
                                }
                                a {
                                    class: "block px-4 py-2 text-sm text-gray-700",
                                    id: "user-menu-item-2",
                                    href: "#",
                                    role: "menuitem",
                                    tabindex: "-1","Sign out"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "sm:hidden",
                id: "mobile-menu",
                div {
                    class: "px-2 pt-2 pb-3 space-y-1",
                    a {
                        class: "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium",
                        "aria-current": "page",
                        href: "#","Dashboard"
                    }
                    a {
                        class: "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium",
                        href: "#","Team"
                    }
                    a {
                        class: "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium",
                        href: "#","Projects"
                    }
                    a {
                        class: "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium",
                        href: "#","Calendar"
                    }
                }
            }
        }
    }
}
