use serde_json::json;
use sycamore::{futures::ScopeSpawnFuture, prelude::*};
use tracing::info;

use crate::{AppState, Controls, VideoStream};

#[component]
pub fn Video<G: Html>(ctx: ScopeRef) -> View<G> {
    let state = ctx.use_context::<AppState>();
    let constraints = ctx.create_selector(|| match state.device_id.get().as_str() {
        "" => json!({
            "facingMode": "user",
        }),
        id => json!({
            "deviceId": { "exact": id },
        }),
    });
    let video_ref = ctx.create_node_ref();
    ctx.create_effect(move || {
        constraints.track();
        ctx.spawn_future(async move {
            info!("future spawned: {:?}", constraints.get());
            let el = video_ref.get::<DomNode>().unchecked_into();
            let video_stream = VideoStream::new(el);
            video_stream.set_video_src(&constraints.get()).await;
        });
    });

    let show_controls = ctx.create_signal(true);

    view! {ctx,
        div(
            class="relative",
            on:mouseover = move |_| show_controls.set(true),
            on:mouseout = move |_| show_controls.set(false),
        ) {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width={state.get_width()},
                height={state.get_height()},
            )
            Controls(show_controls)
        }
    }
}
