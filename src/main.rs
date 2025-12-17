//! Entropy Forge - GUI Application

use eframe::egui;
use entropy_forge::viz::EntropyWeaverApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("Entropy Weaver")
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Entropy Weaver",
        options,
        Box::new(|_cc| {
            Ok(Box::new(EntropyWeaverApp::default()))
        }),
    )
}

fn load_icon() -> egui::IconData {
    // Simple 32x32 icon (can be replaced with actual icon)
    let icon_size = 32;
    let mut rgba = vec![0u8; icon_size * icon_size * 4];

    // Create a simple pattern (can be customized)
    for y in 0..icon_size {
        for x in 0..icon_size {
            let idx = (y * icon_size + x) * 4;
            let value = ((x + y) * 8) as u8;
            rgba[idx] = value;     // R
            rgba[idx + 1] = value; // G
            rgba[idx + 2] = value; // B
            rgba[idx + 3] = 255;   // A
        }
    }

    egui::IconData {
        rgba,
        width: icon_size as u32,
        height: icon_size as u32,
    }
}
