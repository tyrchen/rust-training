mod controls;
mod video;

use crate::AppState;
use sycamore::prelude::*;

pub use controls::Controls;
pub use video::Video;

#[component]
pub async fn App<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let state = AppState::new().await;
    ctx.provide_context(state);
    view! { ctx,
        div(class="container") {
            Video()
        }
    }
}
