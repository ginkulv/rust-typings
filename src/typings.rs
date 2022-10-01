use eframe::{
    egui::{Color32, Layout, TextEdit, Context, FontDefinitions, Ui, Key, RichText, TextStyle},
    emath::Align
};
use std::process;
use std::time::Instant;
use rand::{seq::IteratorRandom, thread_rng};

const FONT_SIZE: f32 = 20.;
const SAMPLE_SIZE: usize = 2;

enum Highlight {
    CORRECT,
    WRONG,
    NONE,
    NEXT,
    TYPO,
}

pub struct Typings {
    words: Vec<Word>,
    value: String,
    cur_index: usize,
    words_typed: usize,
    word_progress: usize,
}

struct Word {
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

fn load_words() -> Vec<Word> {
    let bytes = include_bytes!("../res/words_en.txt");
    let mut file = String::from_utf8_lossy(bytes);
    let file = file.to_mut();
    let words = file.as_str().split("\r\n").filter(|w| w.to_string() != "").map(|w| {
        Word {
            value: w.to_string(),
            highlight: Highlight::NONE
        }
    });
    let mut rng = thread_rng();
    words.choose_multiple(&mut rng, SAMPLE_SIZE).into_iter().collect()
}

impl Typings {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        // TODO it would be more efficient to load words only once and just get a sample of it
        let mut words = load_words();
        words[0].highlight = Highlight::NEXT;
        Self {
            value: "".to_owned(),
            words,
            cur_index: 0,
            words_typed: 0,
            word_progress: 0,
        }
    }

    pub fn render_words(&self, ui: &mut Ui) {
        ui.add_space(20.);
        let mut current_width: f32 = 0.;
        let mut i: usize = 0;
        let screen_width = ui.max_rect().width() * 0.7;
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
                            Highlight::TYPO    => Color32::from_rgb(116, 40, 2),
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

    pub fn render_labels(&mut self, ui: &mut Ui) {
        ui.add_space(40.);
        println!("we're here");
        ui.label(format!("Words: {}", self.words_typed));
    }

    pub fn render_input(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add_space(38.);
                let input = TextEdit::singleline(&mut self.value)
                    .lock_focus(true)
                    .font(TextStyle::Heading);
                let response = ui.add_sized([200., 20.], input);
                response.request_focus();

                if ui.input().key_pressed(Key::Escape) {
                    process::exit(0);
                }

                if ui.input().key_pressed(Key::Tab) {
                    let mut words = load_words();
                    words[0].highlight = Highlight::NEXT;
                    self.value = "".to_owned();
                    self.words = words;
                    self.cur_index = 0;
                    self.words_typed = 0;
                    self.word_progress = 0;
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

                    self.word_progress += self.words[self.cur_index].value.len();
                    self.words_typed += self.word_progress / 5;
                    self.word_progress = self.word_progress - (self.word_progress / 5);
                    self.words_typed += 1;
                    self.word_progress = 0;
                    self.value = "".to_string();
                    self.cur_index += 1;

                    if self.cur_index < self.words.len() {
                        self.words[self.cur_index].highlight = Highlight::NEXT;
                    }

                    return;
                }

                if response.changed() && self.cur_index < self.words.len() {
                    if self.words[self.cur_index].value.starts_with(&self.value) {
                        self.words[self.cur_index].highlight = Highlight::NEXT;
                    } else {
                        self.words[self.cur_index].highlight = Highlight::TYPO;
                    }

                }

            });
        });
    }
}
