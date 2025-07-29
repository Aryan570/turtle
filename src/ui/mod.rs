use eframe::{
    App, Frame,
    egui::{self, Area, Id, Rect, TextEdit, Vec2},
};
use std::{ffi::OsString, mem, os::windows::ffi::OsStrExt};
use urlencoding::encode;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HRESULT, PCWSTR};
const INPUT_SIZE: eframe::egui::Vec2 = Vec2::new(600.0, 40.0);

fn search_browser(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut url_wstr: Vec<u16> = OsString::from(url).encode_wide().collect();
    url_wstr.push(0);
    let mut operation: Vec<u16> = OsString::from("open").encode_wide().collect();
    operation.push(0);
    unsafe {
        let file_path = PCWSTR::from_raw(url_wstr.as_ptr());
        let op_ptr = PCWSTR::from_raw(operation.as_ptr());
        let result = ShellExecuteW(
            Some(HWND(std::ptr::null_mut())),
            op_ptr,
            file_path,
            PCWSTR::null(),
            PCWSTR::null(),
            SHOW_WINDOW_CMD(SW_SHOWNORMAL.0),
        );
        if result.0 as isize <= 32 {
            let hr = HRESULT::from_win32(result.0 as u32);
            return Err(format!(
                "Failed to open URL. ShellExecuteW returned error code: {}. HRESULT: {:?}",
                result.0 as isize, hr
            )
            .into());
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct Input {
    commands: String,
}
impl App for Input {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            let cmds = mem::take(&mut self.commands);
            let encoded = encode(&cmds);
            let url = format!("https://www.google.com/search?q={}", encoded);
            match search_browser(&url) {
                Ok(_) => println!("Browser opened successfully"),
                Err(e) => {
                    eprintln!("Something went wrong! => {}", e);
                }
            }
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
                let text_edit = TextEdit::singleline(&mut self.commands)
                    .id(id)
                    .desired_width(INPUT_SIZE.x)
                    .lock_focus(true)
                    .cursor_at_end(true)
                    .hint_text("Search or run command…")
                    .font(egui::TextStyle::Heading);
                let response = ui.add(text_edit);
                response.request_focus();
            });
    }
}
