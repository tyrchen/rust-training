use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

use crate::AppState;

#[component]
pub fn Controls<'a, G: Html>(ctx: ScopeRef<'a>, show_controls: &'a Signal<bool>) -> View<G> {
    let state = ctx.use_context::<AppState>();
    let devices = ctx.create_memo(|| state.devices.get().video_devices().cloned().collect());
    let visible = ctx.create_memo(move || match *show_controls.get() {
        true => "visible",
        false => "invisible",
    });
    let class = || format!("absolute bottom-2 p-5 {}", visible.get());
    view! {ctx,
        div(class=class()) {
            div(class="flex justify-center") {
                div(class="xl:w-1/3") {
                    select(class="form-select appearance-none
                        block
                        px-3
                        py-1.5
                        text-base
                        font-normal
                        text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300
                        rounded
                        transition
                        ease-in-out
                        m-0
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                        on:change=|e: Event| {
                            let target: HtmlSelectElement = e.target().unwrap().unchecked_into();
                            let device_id = target.value();
                            state.device_id.set(device_id);
                        }) {
                            Keyed {
                                iterable: devices,
                                view: |ctx, device| view!{ctx, option(value=device.id) {
                                    (device.label)
                                }},
                                key: |device| device.id.clone(),
                            }
                    }
                }
            }
        }
    }
}
