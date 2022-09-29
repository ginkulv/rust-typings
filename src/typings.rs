use eframe::{
    egui::{Color32, Layout, TextEdit, Context, FontDefinitions, Ui, Key, RichText, TextStyle},
    emath::Align
};
use std::process;

const FONT_SIZE: f32 = 20.;

pub enum Highlight {
    CORRECT,
    WRONG,
    NONE,
    NEXT,
}

pub struct Typings {
    words: Vec<Word>,
    value: String,
    cur_index: usize,
}

pub struct Word {
    value: String,
    highlight: Highlight,
}

fn setup_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts
        .families
        .entry(eframe::epaint::FontFamily::Proportional);

    ctx.set_fonts(fonts);
}

impl Typings {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        let iter = (0..20).map(|a| Word {
            value: format!("title{}", a),
            highlight: Highlight::NONE,
        });
        let mut words = Vec::from_iter(iter);
        words[0].highlight = Highlight::NEXT;
        Self {
            value: "".to_owned(),
            words,
            cur_index: 0
        }
    }

    pub fn render_words(&self, ui: &mut Ui) {
        let mut current_width: f32 = 0.;
        let mut i: usize = 0;
        let screen_width = ui.max_rect().width() * 0.6;
        ui.vertical_centered(|ui| {
            while i != self.words.len() {
                ui.horizontal(|ui| {
                    // Didn't really get it to align properly, we'll see
                    ui.add_space(50.);
                    while i != self.words.len() && current_width < screen_width {
                        let color = match self.words[i].highlight {
                            Highlight::NONE    => Color32::from_rgb(255, 255, 255),
                            Highlight::CORRECT => Color32::from_rgb(34, 139, 34),
                            Highlight::WRONG   => Color32::from_rgb(227, 11, 92),
                            Highlight::NEXT    => Color32::from_rgb(137, 207, 240),
                        };
                        let value = RichText::from(&self.words[i].value).size(FONT_SIZE);
                        let label = ui.colored_label(color, value);
                        current_width += label.rect.width();
                        i += 1;
                    }
                });
                current_width = 0.;
            }
        });
    }

    pub fn render_input(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(50.);
            let input = TextEdit::singleline(&mut self.value)
                .lock_focus(true)
                .font(TextStyle::Heading);
            let response = ui.add_sized([200., 20.], input);
            response.request_focus();

            if ui.input().key_pressed(Key::Escape) {
                process::exit(0);
            }

            if ui.input().key_pressed(Key::Tab) {
                let iter = (0..20).map(|a| Word {
                    value: format!("title{}", a),
                    highlight: Highlight::NONE,
                });
                self.value = "".to_owned();
                self.words = Vec::from_iter(iter);
                self.cur_index = 0;

                return;
            }

            if response.changed() && ui.input().key_pressed(Key::Space) {
                if self.cur_index > self.words.len() - 1 {
                    self.value = "".to_string();
                    return;
                }
                let inp_value = &self.value[0..self.value.len() - 1];
                if inp_value == self.words[self.cur_index].value {
                    self.words[self.cur_index].highlight = Highlight::CORRECT;
                } else {
                    self.words[self.cur_index].highlight = Highlight::WRONG;
                }

                self.value = "".to_string();
                self.cur_index += 1;

                if self.cur_index < self.words.len() {
                    self.words[self.cur_index].highlight = Highlight::NEXT;
                }

                return;
            }

        });
    }
}
