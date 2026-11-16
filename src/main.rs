//! Entropy Forge - GUI Application

use eframe::egui;
use entropy_forge::viz::{icon, EntropyForgeApp};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("Entropy Forge")
            .with_icon(icon::icon_data()),
        ..Default::default()
    };

    eframe::run_native(
        "Entropy Forge",
        options,
        Box::new(|_cc| Ok(Box::new(EntropyForgeApp::default()))),
    )
}
