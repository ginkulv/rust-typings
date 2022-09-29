mod typings;

use typings::Typings;
use eframe::{run_native, NativeOptions, egui::{CentralPanel, Vec2}, App, epaint::Pos2};

impl App for Typings {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render_words(ui);
            self.render_input(ui);
        });
    }
}

fn main() {
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 200.));
    // I have no idea how to set position dynamically
    win_options.initial_window_pos = Some(Pos2::new(700., 850.));
    win_options.decorated = false;
    run_native (
        "Typings",
        win_options,
        Box::new(|cc| Box::new(Typings::new(cc))),
    )
}
