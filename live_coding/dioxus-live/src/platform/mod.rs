use crate::{Filter, Todos};

#[cfg(feature = "desktop")]
mod desktop;

#[cfg(feature = "web")]
mod web;

pub trait Store {
    fn get(&self) -> Todos;
    fn set(&self, items: &Todos);
}

#[cfg(feature = "web")]
pub use web::get_store;

#[cfg(feature = "desktop")]
pub use desktop::get_store;

#[cfg(feature = "web")]
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

#[cfg(not(feature = "web"))]
impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}
