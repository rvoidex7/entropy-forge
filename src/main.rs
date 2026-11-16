//! Entropy Forge - GUI Application

// Release builds are a desktop program, not a console tool: without this,
// double-clicking the .exe on Windows opens a terminal window behind the GUI.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use entropy_forge::viz::{icon, EntropyForgeApp};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("Entropy Forge")
            // Wayland takes the taskbar icon from the .desktop file that
            // matches this app id; the window icon alone is ignored there.
            .with_app_id("entropy-forge")
            .with_icon(icon::icon_data()),
        ..Default::default()
    };

    eframe::run_native(
        "Entropy Forge",
        options,
        Box::new(|_cc| Ok(Box::new(EntropyForgeApp::default()))),
    )
}
