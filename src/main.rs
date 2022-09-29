mod typings;

use typings::Typings;

use eframe::{run_native, NativeOptions, egui::{CentralPanel, Vec2, Ui}, App};

impl App for Typings {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            self.render_words(ui);
            self.render_input(ui);
        });
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Typings");
    });
    ui.add_space(10.);
}


fn main() {
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 200.));
    win_options.decorated = false;
    run_native (
        "Typings",
        win_options,
        Box::new(|cc| Box::new(Typings::new(cc))),
    )
}
