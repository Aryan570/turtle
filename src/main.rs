mod ui;
use eframe::{NativeOptions, egui::ViewportBuilder};
use ui::Input;
fn main() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("turtle")
            .with_decorations(false)
            .with_transparent(true)
            .with_maximized(true)
            .with_taskbar(false),
        ..Default::default()
    };
    eframe::run_native(
        "turtle",
        native_options,
        Box::new(|_c| Ok(Box::new(Input::default()))),
    )
}
