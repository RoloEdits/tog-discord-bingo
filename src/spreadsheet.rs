use eframe::egui::Color32;
use std::path::Path;

#[derive(Debug)]
pub struct Row {
    pub(crate) num: u32,
    pub(crate) name: Name,
    pub(crate) guess: String,
    pub(crate) starting_score: i32,
}

#[derive(Debug)]
pub struct Name {
    text: String,
    color: Color32,
}

impl Name {
    pub fn new(text: String, color: Color32) -> Self {
        Self { text, color }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn color(&self) -> Color32 {
        self.color
    }
}

impl Row {
    pub fn num(&self) -> u32 {
        self.num
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn guess(&self) -> &str {
        &self.guess
    }

    pub fn starting_score(&self) -> i32 {
        self.starting_score
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> Vec<Row> {
    let mut contents = Vec::new();

    let workbook = umya_spreadsheet::reader::xlsx::read(path).expect("failed to read xlsx file");

    let worksheet = workbook
        .get_sheet_by_name("Sheet1")
        .expect("failed to read worksheet");

    for row in 1.. {
        let Some(cell) = worksheet.get_cell((1, row)) else {
            break;
        };

        let text = cell.get_cell_value().get_value();

        if text.is_empty() {
            continue;
        }

        let hex = cell
            .get_style()
            .get_font()
            .unwrap()
            .get_color()
            .get_argb_with_theme(workbook.get_theme());

        let color = if hex.len() == 6 {
            if hex == "000000" {
                String::from("#f2f3f5")
            } else {
                format!("#{hex}")
            }
        } else if let Some(hex) = hex.get(2..) {
            format!("#{hex}")
        } else {
            String::from("#f2f3f5")
        };

        let color =
            Color32::from_hex(&color).expect("colors from spreadsheet should always be hex");

        let guess = worksheet.get_value((2, row));
        let score = worksheet.get_value((3, row)).parse().unwrap_or_default();

        contents.push(Row {
            num: row,
            name: Name::new(text.to_string(), color),
            guess,
            starting_score: score,
        });
    }

    contents
}
