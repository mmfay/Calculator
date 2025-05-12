mod ui;

use ui::gui::MyApp;
use eframe;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

