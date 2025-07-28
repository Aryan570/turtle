use eframe::{
    App, Frame,
    egui::{self, Area, Id, Rect, TextEdit, Vec2},
};

#[derive(Default)]
pub struct Input {
    commands: String,
    is_focused: bool,
}
const INPUT_SIZE: eframe::egui::Vec2 = Vec2::new(600.0, 40.0);
impl App for Input {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        let visuals = egui::Visuals {
            window_fill: egui::Color32::TRANSPARENT,
            ..ctx.style().visuals.clone()
        };
        ctx.set_visuals(visuals);
        let full = ctx.available_rect();
        let center = full.center();
        let top_left = center - INPUT_SIZE / 2.0;
        let rect = Rect::from_min_size(top_left, INPUT_SIZE);
        Area::new("input_area".into())
            .constrain_to(rect)
            .show(ctx, |ui| {
                let id = Id::new("spotlight_input");
                if !self.is_focused {
                    ui.memory_mut(|m| m.request_focus(id));
                    self.is_focused = true;
                }
                ui.add(
                    TextEdit::singleline(&mut self.commands)
                        .id(id)
                        .desired_width(INPUT_SIZE.x)
                        .lock_focus(true)
                        .hint_text("Search or run command…")
                        .font(egui::TextStyle::Heading),
                );
            });
    }
}
