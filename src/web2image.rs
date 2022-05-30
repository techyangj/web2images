use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};

use anyhow::{anyhow,Result};
use image::{DynamicImage, load_from_memory, ImageFormat, GenericImageView};

use qrcode::QrCode;
use image::Luma;
use std::{fmt::Display, time::Instant};

use image::imageops::overlay;



fn url2image(url: &str) -> Result<DynamicImage> {

  let start = Instant::now();

  fn to_anyhow(e: impl Display) -> anyhow::Error {
    anyhow!(e.to_string())
  }


  let browser = Browser::new(LaunchOptionsBuilder::default()
    .window_size(Some((1200, 1600)))
    .build().unwrap())
    .map_err(to_anyhow)?;
  let tab = browser.wait_for_initial_tab()
    .map_err(to_anyhow)?;
  let viewport = tab
      .navigate_to(url)
      .map_err(to_anyhow)?
      .wait_for_element("body")
      .map_err(to_anyhow)?
      .get_box_model()
      .map_err(to_anyhow)?
      .margin_viewport();

  dbg!(&viewport);
  let data = tab
    .capture_screenshot(ScreenshotFormat::PNG, Some(viewport), true).map_err(to_anyhow)?;
  
  println!("time spend on url2image: {}ms",start.elapsed().as_millis());

  Ok(load_from_memory(&data)?)

}




fn gen_qrcode(url: &str) -> Result<DynamicImage> {
  let code = QrCode::new(url.as_bytes())?;

  // Render the bits into an image.
  let buf = code.render::<Luma<u8>>().build();

  Ok(DynamicImage::ImageLuma8(buf))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
  // overlay(bottom, top, 0, 0);
  let x = bottom.width() - top.width() - 10;
  let y = bottom.height() - top.height() - 10;
  overlay(bottom, top, x, y);
}


pub fn web2image(url: &str, output: &str, format: ImageFormat) -> Result<()> {
  let mut bottom = url2image(url)?;
  let qrcode = gen_qrcode(url)?;

  do_overlay(&mut bottom, &qrcode);

  bottom.save_with_format(output, format)?;

  Ok(())
}