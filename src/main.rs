// NOTE: Hide console in Windows when using release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bingo::{Bingo, Key, spreadsheet::Row};
use eframe::egui::{Color32, Id, Modal};
use egui_extras::{Column, TableBuilder};
use mimalloc::MiMalloc;
use std::str::FromStr;
use std::{ffi::OsStr, path::PathBuf};

mod fonts;

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

    eframe::run_native(
        "Bingo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts::load());
            cc.egui_ctx.set_pixels_per_point(2.0);
            Ok(Box::<Application>::default())
        }),
    )
    .unwrap();
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
    Normal,
    GreatWar,
}

impl eframe::App for Application {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if self.rows.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.centered_and_justified(|ui| ui.label("Drag-and-drop file onto the window!"))
                });
                if let Some(path) = &self.path {
                    if path.extension() == Some(OsStr::new("xlsx")) {
                        self.rows = bingo::spreadsheet::read(path);
                    } else {
                        let modal = Modal::new(Id::new("IF")).show(ctx, |ui| {
                            ui.set_width(200.0);
                            ui.heading("Invalid Filetype:");
                            ui.separator();
                            ui.label("Must be xslx");
                        });

                        if modal.should_close() {
                            self.path = None;
                        }
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
                            Rules::Normal => Bingo::normal(rows),
                            Rules::GreatWar => Bingo::great_war(rows),
                        };

                        if let Err(err) = bingo {
                            // FIX: Modals don't seem to open, though the closures are called.
                            // For now, no errors will be returned and the onus is on GM to
                            // account for correctness.
                            match err {
                                bingo::error::Error::DoubleGuesser { row, name } => {
                                    Modal::new(Id::new("DG")).show(ctx, |ui| {
                                        ui.set_width(200.0);
                                        ui.heading("Player Guessed More Than Once");
                                        ui.label(format!("{row}: {name}"));
                                        ui.label("Fix the spreadsheet and save, then reload the file by pressing the Reload button, and rescore the bingo");
                                    });
                                }
                                bingo::error::Error::NotEnoughValidSquares {
                                    name,
                                    row,
                                    amount,
                                    needed,
                                } => {
                                    Modal::new(Id::new("NES")).show(ctx, |ui| {
                                        ui.set_width(200.0);
                                        ui.heading("Incorrect Number of Squares");
                                        ui.label(format!("{row}: {name} | Guessed for `{amount}` squares, needs `{needed}` squares "));
                                        ui.label("Fix the spreadsheet and save, then reload the file by pressing the Reload button, and rescore the bingo");
                                    });
                                }
                            }
                        } else if let Ok(mut bingo) = bingo {
                            let key = Key::from_str(&self.key).unwrap();
                            bingo.play(&key);
                            self.bingo = Some(bingo);
                            self.scored = true;
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.rules, Rules::Normal, "Normal");
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
                            self.bingo
                                .as_ref()
                                .unwrap()
                                .save_html(self.path.as_ref().unwrap());
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
                                body.row(18.0, |mut table_row| {
                                    table_row.col(|ui| {
                                        ui.label(row.num().to_string());
                                    });
                                    table_row.col(|ui| {
                                        ui.colored_label(row.name().color(), row.name().text());
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
