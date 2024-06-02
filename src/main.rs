// NOTE: Hide console in Windows when using release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bingo::{Bingo, Key, Player};
use eframe::egui::{Color32, FontDefinitions};
use egui_extras::{Column, TableBuilder};
use egui_modal::Modal;
use std::{ffi::OsStr, path::PathBuf};
mod spreadsheet;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([815.0, 850.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true)
            .with_icon(include_icon(include_bytes!("../icon.png"))),
        ..Default::default()
    };

    let fonts = register_fonts();


    eframe::run_native(
        "Bingo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_pixels_per_point(2.0);
            Box::<Application>::default()
        }),
    )
    .unwrap();
}

fn register_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "ggsans_bold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_bold.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_bold".to_string());
    
    fonts.font_data.insert(
        "ggsans_bolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_bolditalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_bolditalic".to_string());
    
    fonts.font_data.insert(
        "ggsans_extrabold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_extrabold.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_extrabold".to_string());
    
    fonts.font_data.insert(
        "ggsans_extrabolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_extrabolditalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_extrabolditalic".to_string());
    
    fonts.font_data.insert(
        "ggsans_medium".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_medium.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_medium".to_string());
    
    fonts.font_data.insert(
        "ggsans_mediumitalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_mediumitalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_mediumitalic".to_string());
    
    fonts.font_data.insert(
        "ggsans_normal".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_normal.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push(  "ggsans_normal".to_string());
    
    fonts.font_data.insert(
        "ggsans_normalitalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_normalitalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_normalitalic".to_string());
    
    fonts.font_data.insert(
        "ggsans_semibold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_semibold.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0,  "ggsans_semibold".to_string());
   
    fonts.font_data.insert(
        "ggsans_semibolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_semibolditalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "ggsans_semibolditalic".to_string());

    fonts.font_data.insert(
        "seguihis".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/seguihis.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "seguihis".to_string());

    fonts.font_data.insert(
        "seguisym".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/seguisym.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push( "seguisym".to_string());

    fonts
    
}

fn include_icon(icon: &[u8]) -> eframe::egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

#[derive(Default)]
struct Application {
    path: Option<PathBuf>,
    key: String,
    bingo: Bingo,
    scored: bool,
    contents: Vec<(u32, String, String, String)>,
}

impl eframe::App for Application {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut invalid_key_modal = Modal::new(ctx, "key_modal");
        invalid_key_modal.show_dialog();

        let invalid_key_dialog = invalid_key_modal
            .dialog()
            .with_title("Invalid Key")
            .with_body("Provided Key was invalid: must only contain Y or N");

        let mut invalid_filetype_modal = Modal::new(ctx, "filetype_modal");
        invalid_filetype_modal.show_dialog();

        let invalid_filetype_dialog = invalid_filetype_modal
            .dialog()
            .with_title("Invalid Filetype")
            .with_body("Provided File was invalid: must be .xslx spreadsheet");

        let mut double_guesser_modal = Modal::new(ctx, "double_guesser_modal");
        double_guesser_modal.show_dialog();

        let double_guesser_dialog = double_guesser_modal.dialog().with_title("Multi-Guessers");

        let mut invalid_guess_modal = Modal::new(ctx, "invalid_guess_modal");
        invalid_guess_modal.show_dialog();

        let invalid_guessers_dialog = invalid_guess_modal.dialog().with_title("Invalid Guess");

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if self.contents.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.centered_and_justified(|ui| ui.label("Drag-and-drop file onto the window!"))
                });
                if let Some(path) = &self.path {
                    if path.extension() == Some(OsStr::new("xlsx")) {
                        self.contents = spreadsheet::read(path);
                    } else {
                        invalid_filetype_dialog.open();
                        self.path = None;
                    }
                }
            } else {
                ui.horizontal(|ui| {
                    let key_label = ui.label("Key: ");

                    ui.text_edit_singleline(&mut self.key)
                        .labelled_by(key_label.id);

                    self.key = key_fmt(&self.key);

                    ui.add_enabled_ui(
                        self.key
                            .chars()
                            .filter(|char| !char.is_whitespace())
                            .count()
                            == 12,
                        |ui| {
                            let hover_text = |ui: &mut eframe::egui::Ui| {
                                ui.label("Key must be for all 12 squares and only contain Y or N");
                            };

                            if ui
                                .button("Score")
                                .on_disabled_hover_ui(hover_text)
                                .clicked()
                            {
                                self.bingo.clear_players();

                                let mut double_guessers = Vec::new();

                                let mut invalid_guessers = Vec::new();

                                for (row, name, color, guess) in &self.contents {
                                    // NOTE: When a cell is entered, but then deleted, it can leave behind a residual
                                    // empty string.

                                    if name.is_empty() {
                                        continue;
                                    }

                                    let player = Player::new(
                                        *row,
                                        name.clone(),
                                        color.clone(),
                                        guess.clone(),
                                    );

                                    if let Ok(player) = player {
                                        if let Err(err) = self.bingo.add_player(player) {
                                            match err {
                                                bingo::BingoError::DoubleGuesser {
                                                    name,
                                                    row,
                                                    guess,
                                                } => {
                                                    double_guessers.push((row, name, guess));
                                                }
                                            }
                                        }
                                    } else {
                                        invalid_guessers.push((row, name, guess));
                                    }
                                }

                                if !invalid_guessers.is_empty() {
                                    let mut string = String::new();

                                    for (row, name, guess) in invalid_guessers {
                                        string.push_str(&row.to_string());
                                        string.push_str(" | ");
                                        string.push_str(name);
                                        string.push_str(" | ");
                                        string.push_str(guess);
                                        string.push('\n');
                                        string.push('\n');
                                    }

                                    string.push_str("Fix the spreadsheet and save, then reload the file by pressing the Reload button, and rescore the bingo");

                                    let dialog = invalid_guessers_dialog.with_body(string);

                                    dialog.open();
                                }

                                if !double_guessers.is_empty() {
                                    let mut string = String::new();

                                    for (row, name, guess) in double_guessers {
                                        string.push_str(&row.to_string());
                                        string.push_str(" | ");
                                        string.push_str(&name);
                                        string.push_str(" | ");
                                        string.push_str(&guess);
                                        string.push('\n');
                                        string.push('\n');
                                    }
                                    
                                    string.push_str("Fix the spreadsheet and save, then reload the file by pressing the Reload button, and rescore the bingo");

                                    let dialog = double_guesser_dialog.with_body(string);

                                    dialog.open();
                                }

                                if let Ok(key) = Key::new(&self.key) {
                                    self.bingo.score(&key);
                                    self.scored = true;
                                } else {
                                    invalid_key_dialog.open();
                                }
                            }
                        },
                    );
                });

                if !self.scored {
                    ui.separator();

                    if ui.button("Reload File").clicked() {
                        if let Some(path) = &self.path {
                            self.contents = spreadsheet::read(path);
                            self.scored = false;
                        }
                    }

                    ui.separator();
                }

                let available_height = ui.available_height();

                if self.scored {
                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Reload File").clicked() {
                            if let Some(path) = &self.path {
                                self.contents = spreadsheet::read(path);
                                self.scored = false;
                            }
                        }

                        if ui.button("Save").clicked() {
                            self.bingo.save_html(
                                self.path
                                    .as_ref()
                                    .expect("path should have been given at this point")
                                    .clone(),
                            );
                        }
                    });

                    ui.separator();

                    let table = TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(eframe::egui::Layout::left_to_right(
                            eframe::egui::Align::Center,
                        ))
                        .column(Column::exact(322.0))
                        .column(Column::exact(36.0))
                        .min_scrolled_height(0.0)
                        .max_scroll_height(available_height);

                    table
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Name");
                            });
                            header.col(|ui| {
                                ui.strong("Score");
                            });
                        })
                        .body(|mut body| {
                            for player in &self.bingo.players {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.colored_label(Color32::from_hex(&player.color).expect("colors from spreadsheet should always be hex"),player.name.clone());
                                    });
                                    row.col(|ui| {
                                        ui.label(player.score.to_string());
                                    });
                                });
                            }
                        });
                } else {
                    let table = TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(eframe::egui::Layout::left_to_right(
                            eframe::egui::Align::Center,
                        ))
                        .column(Column::auto())
                        .column(Column::initial(200.0))
                        .column(Column::auto())
                        .min_scrolled_height(0.0)
                        .max_scroll_height(available_height);
                    table
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Row");
                            });
                            header.col(|ui| {
                                ui.strong("Name");
                            });
                            header.col(|ui| {
                                ui.strong("Guess");
                            });
                        })
                        .body(|mut body| {
                            for (rw, name, color, guess) in &self.contents {
                                if name.is_empty() {
                                    continue;
                                }

                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(rw.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.colored_label(Color32::from_hex(color).expect("colors from spreadsheet should always be hex"),name);
                                    });
                                    row.col(|ui| {
                                        ui.label(format_guess(guess));
                                    });
                                });
                            }
                        });
                }
            }
        });

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.path.clone_from(&i.raw.dropped_files[0].path);
            }
        });
    }
}

fn key_fmt(key: &str) -> String {
    key.to_uppercase().replace('\n', " ")
}

fn format_guess(guess: &str) -> String {
    let mut result = String::with_capacity(12 + 4);

    for (count, ch) in guess.to_uppercase().replace(['\n' ,'/', 'A', 'B', 'C', ':', ' ', '.'], "").chars().enumerate() {
        if count > 0 && count % 4 == 0 {
            result.push(' ');
        }
        result.push(ch);
    }

    result
}
