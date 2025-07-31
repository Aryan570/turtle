mod ui;
use eframe::{NativeOptions, egui::ViewportBuilder};
use ui::Input;
fn main() -> eframe::Result<()> {
    let mut font_def = eframe::egui::FontDefinitions::default();
    font_def.font_data.insert(
        String::from("Englebert"),
        eframe::egui::FontData::from_static(include_bytes!("../assets/Englebert-Regular.ttf"))
            .into(),
    );
    font_def
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, String::from("Englebert"));
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
        Box::new(move |c| {
            c.egui_ctx.set_fonts(font_def);
            Ok(Box::new(Input::default()))
        }),
    )
}
