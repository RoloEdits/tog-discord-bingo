// NOTE: Hide console in Windows when using release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bingo::{Bingo, Key, spreadsheet::Row};
use eframe::egui::{Color32, FontDefinitions};
use egui_extras::{Column, TableBuilder};
use egui_modal::Modal;
use std::{ffi::OsStr, path::PathBuf};
use std::str::FromStr;
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
            Ok(Box::<Application>::default())
        }),
    )
    .unwrap();
}

#[allow(clippy::too_many_lines)]
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
        .push("ggsans_bolditalic".to_string());

    fonts.font_data.insert(
        "ggsans_extrabold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_extrabold.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_extrabold".to_string());

    fonts.font_data.insert(
        "ggsans_extrabolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_extrabolditalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_extrabolditalic".to_string());

    fonts.font_data.insert(
        "ggsans_medium".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_medium.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_medium".to_string());

    fonts.font_data.insert(
        "ggsans_mediumitalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_mediumitalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_mediumitalic".to_string());

    fonts.font_data.insert(
        "ggsans_normal".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_normal.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_normal".to_string());

    fonts.font_data.insert(
        "ggsans_normalitalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_normalitalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_normalitalic".to_string());

    fonts.font_data.insert(
        "ggsans_semibold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_semibold.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "ggsans_semibold".to_string());

    fonts.font_data.insert(
        "ggsans_semibolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_semibolditalic.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_semibolditalic".to_string());

    fonts.font_data.insert(
        "seguihis".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/seguihis.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("seguihis".to_string());

    fonts.font_data.insert(
        "seguisym".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/seguisym.ttf")),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("seguisym".to_string());

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
    bingo: Option<Bingo>,
    scored: bool,
    rules: Rules,
    rows: Vec<Row>,
}

#[derive(PartialEq, Eq, Default)]
enum Rules {
    #[default]
    Default,
    GreatWar,
}

impl eframe::App for Application {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut invalid_key_modal = Modal::new(ctx, "key_modal");
        invalid_key_modal.show_dialog();

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
            if self.rows.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.centered_and_justified(|ui| ui.label("Drag-and-drop file onto the window!"))
                });
                if let Some(path) = &self.path {
                    if path.extension() == Some(OsStr::new("xlsx")) {
                        self.rows = bingo::spreadsheet::read(path);
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

                    if ui.button("Score").clicked() {
                        let rows = self.rows.as_slice();
                        let bingo = match self.rules {
                            Rules::Default => Bingo::normal(rows),
                            Rules::GreatWar => Bingo::great_war(rows)
                        };

                        if let Err(err) = bingo {
                            match err {
                                bingo::error::Error::DoubleGuesser { row } => {
                                    let mut body = String::new();
                                    
                                    body.push_str(&row.to_string());
                                    body.push('\n');
                                    body.push('\n');
                                    body.push_str("Fix the spreadsheet and save, then reload the file by pressing the Reload button, and rescore the bingo");

                                    let dialog = invalid_guessers_dialog.with_body(body);

                                    dialog.open();
                                },
                                bingo::error::Error::NotEnoughValidSquares { name, row, amount, needed } => {
                                    let mut body = String::new();

                                    body.push_str(&row.to_string());
                                    body.push_str(" | ");
                                    body.push_str(&name);
                                    body.push_str(" | ");
                                    body.push('\n');
                                    body.push('\n');

                                    let message = format!("Only guessed for `{amount}` squares, needs `{needed}` squares");
                                    body.push_str(&message);
                                    body.push('\n');
                                    
                                    body.push_str("Fix the spreadsheet and save, then reload the file by pressing the Reload button, and rescore the bingo");

                                    let dialog = double_guesser_dialog.with_body(body);

                                    dialog.open();
                                },
                            }
                        } else if let Ok( mut bingo) = bingo {
                            let key = Key::from_str(&self.key).expect("should be infallible");
                            bingo.play(&key);
                            self.bingo = Some(bingo);
                            self.scored = true;
                        }
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.rules, Rules::Default, "Default");
                    ui.radio_value(&mut self.rules, Rules::GreatWar, "Great War");
                });
                
                if !self.scored {
                    ui.separator();

                    if ui.button("Reload File").clicked() {
                        if let Some(path) = &self.path {
                            self.rows = bingo::spreadsheet::read(path);
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
                                self.rows = bingo::spreadsheet::read(self.path.as_ref().unwrap());
                                self.scored = false;
                        }

                        if ui.button("Save").clicked() {
                            self.bingo.as_ref().unwrap().save_html(
                                self.path
                                    .as_ref()
                                    .expect("path should have been given at this point"),
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
                            for player in self.bingo.as_ref().unwrap().players() {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.colored_label(
                                            Color32::from_hex(&player.color).expect(
                                                "colors from spreadsheet should always be hex",
                                            ),
                                            player.name.clone(),
                                        );
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
                            for row in &self.rows {
                                if row.name().is_empty() {
                                    continue;
                                }

                                body.row(18.0, |mut table_row| {
                                    table_row.col(|ui| {
                                        ui.label(row.num().to_string());
                                    });
                                    table_row.col(|ui| {
                                        ui.colored_label(
                                            row.name().color(),
                                        row.name().text(),
                                        );
                                    });
                                    table_row.col(|ui| {
                                        ui.label(row.guess());
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
