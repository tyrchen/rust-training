use tracing::warn;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStream, MediaStreamConstraints};

use crate::Devices;
pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub async fn set_video_src(&self, video_constraints: &serde_json::Value) {
        let devices = Devices::get_media_devices().await;

        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_serde(video_constraints).unwrap());
        constraints.audio(&false.into());
        let media = JsFuture::from(
            devices
                .get_user_media_with_constraints(&constraints)
                .unwrap(),
        )
        .await
        .unwrap();
        match media.dyn_ref::<MediaStream>() {
            Some(stream) => {
                self.el.set_src_object(Some(&stream));
            }
            None => {
                warn!("No media stream found");
            }
        }
    }
}
