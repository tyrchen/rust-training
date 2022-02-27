use std::ops::Deref;

use crate::Todos;

use super::Store;
use web_sys::Storage;

const TODO_KEY: &str = "todos_dioxus";
pub struct LocalStorage(Storage);

impl Deref for LocalStorage {
    type Target = Storage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self(
            web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .expect("User did not allow local storage"),
        )
    }
}

impl Store for LocalStorage {
    fn get(&self) -> Todos {
        if let Ok(Some(content)) = self.get_item(TODO_KEY) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn set(&self, todos: &Todos) {
        let content = serde_json::to_string(todos).unwrap();
        self.set_item(TODO_KEY, &content).unwrap();
    }
}

pub fn get_store() -> impl Store {
    LocalStorage::default()
}
