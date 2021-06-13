use std::{fmt::Display, thread, time::Instant};

use anyhow::{anyhow, Result};
use headless_chrome::{
    protocol::{page::ScreenshotFormat, target::methods::CreateTarget},
    Browser, LaunchOptionsBuilder,
};
use image::{
    imageops::overlay, load_from_memory, DynamicImage, GenericImageView, ImageFormat, Luma,
};
use qrcode::QrCode;

// unfortunately we can't do this since we don't own either Fail/Error trait
// impl From<dyn failure::Fail> for Box<dyn std::error::Error + 'static> {
//     fn from(_: dyn failure::Fail) -> Self {
//         ...
//     }
// }

fn url2image(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    fn to_anyhow(e: impl Display) -> anyhow::Error {
        anyhow!(e.to_string())
    }

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .window_size(Some((1200, 1600)))
            .build()
            .unwrap(),
    )
    .map_err(to_anyhow)?;
    let tab = browser.wait_for_initial_tab().map_err(to_anyhow)?;
    let viewport = tab
        .navigate_to(url)
        .map_err(to_anyhow)?
        .wait_until_navigated()
        .map_err(to_anyhow)?
        .find_element("body")
        .map_err(to_anyhow)?
        .get_box_model()
        .map_err(to_anyhow)?
        .margin_viewport();

    dbg!(&viewport);

    // this is a hack for headless chrome as it cannot handle the case that
    // viewport is bigger than window size. I guess pupeeteer have a solution
    // there but for this quick-and-dirty live coding, let's just open a new
    // tab and set its width/height.
    let tab = browser
        .new_tab_with_options(CreateTarget {
            url,
            width: Some(viewport.width as i32),
            height: Some(viewport.height as i32 + 10),
            browser_context_id: None,
            enable_begin_frame_control: None,
        })
        .map_err(to_anyhow)?;
    tab.wait_until_navigated().map_err(to_anyhow)?;

    let data = tab
        .capture_screenshot(ScreenshotFormat::PNG, None, true)
        .map_err(to_anyhow)?;
    println!("time spent on url2image: {}ms", start.elapsed().as_millis());
    Ok(load_from_memory(&data)?)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    let code = QrCode::new(url.as_bytes())?;

    // Render the bits into an image.
    let buf = code.render::<Luma<u8>>().build();
    println!(
        "time spent on gen_qrcode: {}ms",
        start.elapsed().as_millis()
    );
    Ok(DynamicImage::ImageLuma8(buf))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let start = Instant::now();
    let x = bottom.width() - top.width() - 10;
    let y = bottom.height() - top.height() - 10;
    overlay(bottom, top, x, y);
    println!(
        "time spent on do_overlay: {}ms",
        start.elapsed().as_millis()
    );
}

pub fn web2image(url: &str, output: &str, format: ImageFormat) -> Result<()> {
    let url = url.to_owned();
    let url1 = url.clone();
    let bottom_handle = thread::spawn(move || url2image(&url).unwrap());
    let qrcode_handle = thread::spawn(move || gen_qrcode(&url1).unwrap());
    let mut bottom = bottom_handle.join().unwrap();
    let qrcode = qrcode_handle.join().unwrap();
    do_overlay(&mut bottom, &qrcode);

    let start = Instant::now();
    bottom.save_with_format(output, format)?;
    println!("time spent on web2image: {}ms", start.elapsed().as_millis());
    Ok(())
}
