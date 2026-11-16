//! Bundled application icon.
//!
//! The same PNG feeds the operating-system window icon (taskbar, alt-tab,
//! window list) and the in-app header, so the program is recognizable outside
//! and inside its own window.

use eframe::egui;

/// 256x256 export of `assets/icons/source/icon.png`.
///
/// Large enough for HiDPI window icons, small enough to decode on startup.
const ICON_PNG: &[u8] = include_bytes!("../../assets/icons/128x128@2x.png");

/// Decode the bundled icon into pixels usable by egui.
pub fn color_image() -> egui::ColorImage {
    let image = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png)
        .expect("bundled application icon is a valid PNG")
        .into_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw())
}

/// Icon data for [`egui::ViewportBuilder::with_icon`].
pub fn icon_data() -> egui::IconData {
    let image = color_image();
    egui::IconData {
        rgba: image
            .pixels
            .iter()
            .flat_map(|pixel| [pixel.r(), pixel.g(), pixel.b(), pixel.a()])
            .collect(),
        width: image.size[0] as u32,
        height: image.size[1] as u32,
    }
}
