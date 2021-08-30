use std::fs;
use std::fmt::Display;

use headless_chrome::{Browser, LaunchOptionsBuilder, protocol::page::ScreenshotFormat};

use anyhow::{anyhow, Result};

fn to_anyhow(err: impl Display) -> anyhow::Error {
    anyhow!(err.to_string())
}

fn url2image(url: &str) -> Result<Vec<u8>> {
    let browser = Browser::new(LaunchOptionsBuilder::default().build().unwrap()).map_err(to_anyhow)?;
    let tab = browser.wait_for_initial_tab().map_err(to_anyhow)?;
    let viewport = tab.navigate_to(url).map_err(to_anyhow)?
        .wait_for_element("body").map_err(to_anyhow)?
        .get_box_model().map_err(to_anyhow)?
        .margin_viewport();
    let png_data = tab.capture_screenshot(ScreenshotFormat::PNG, Some(viewport), true).map_err(to_anyhow)?;
    Ok(png_data)
}

pub fn web2image(url: &str, output: &str) {
    match url2image(url) {
        Ok(data) => fs::write(output, data).unwrap(),
        Err(err) => println!("{}", err.to_string())
    }
}