#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod app;
mod chat_dock;
mod settings_view;
mod theme;
mod views;

use app::KittyCraftApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1360.0, 860.0])
            .with_min_inner_size([960.0, 620.0])
            .with_title("KittyCraft - Katzen- & Kitten-Berater"),
        ..Default::default()
    };
    eframe::run_native(
        "KittyCraft",
        options,
        Box::new(|cc| Ok(Box::new(KittyCraftApp::new(cc)))),
    )
}
