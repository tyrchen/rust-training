mod controls;
mod video;

use crate::AppState;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::{intern, prelude::*, JsCast};

pub use controls::Controls;
pub use video::Video;
use web_sys::Event;

#[component]
pub async fn App<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let state = AppState::new().await;
    ctx.provide_context(state);

    window_event_lisener(
        ctx,
        "resize",
        Box::new(move |_| {
            let state = ctx.use_context::<AppState>();
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            state.dimensions.set((width, height));
            info!("Window resize: {}x{}", width, height);
        }),
    );

    view! { ctx,
        div {
            Video()
        }
    }
}

fn window_event_lisener<'a>(ctx: ScopeRef<'a>, event: &str, callback: Box<dyn Fn(Event) + 'a>) {
    let window = web_sys::window().unwrap();
    let handler: Box<dyn Fn(Event) + 'static> = unsafe { std::mem::transmute(callback) };
    let callback = Closure::wrap(handler);
    window
        .add_event_listener_with_callback(intern(event), callback.as_ref().unchecked_ref())
        .unwrap_throw();

    ctx.on_cleanup(move || {
        drop(callback);
    });
}

// fn event<'a>(&self, ctx: ScopeRef<'a>, name: &str, handler: Box<dyn Fn(Self::EventType) + 'a>) {
//     // SAFETY: extend lifetime because the closure is dropped when the ctx is disposed,
//     // preventing the handler from ever being accessed after its lifetime.
//     let handler: Box<dyn Fn(Self::EventType) + 'static> = unsafe { std::mem::transmute(handler) };
//     let closure = Closure::wrap(handler);
//     self.node
//         .add_event_listener_with_callback(intern(name), closure.as_ref().unchecked_ref())
//         .unwrap_throw();

//     ctx.on_cleanup(move || {
//         drop(closure);
//     });
// }
