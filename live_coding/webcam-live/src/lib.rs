mod components;
mod devices;
mod video_stream;

pub use components::*;
pub use devices::Devices;
use sycamore::prelude::*;
pub use video_stream::VideoStream;

#[derive(Debug)]
pub struct AppState {
    pub dimensions: RcSignal<(u32, u32)>,
    pub device_id: RcSignal<String>,
    pub devices: RcSignal<Devices>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            dimensions: create_rc_signal((800, 450)),
            device_id: create_rc_signal("".into()),
            devices: create_rc_signal(Devices::load().await),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.dimensions.get().0
    }

    pub fn get_height(&self) -> u32 {
        self.dimensions.get().1
    }
}
