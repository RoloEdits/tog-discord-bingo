use std::path::Path;

pub fn read<P: AsRef<Path>>(path: P) -> Vec<(u32, String, String, String)> {
    let mut contents = Vec::new();

    let workbook = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let worksheet = workbook
        .get_sheet_by_name("Sheet1")
        .expect("failed to read worksheet");

    for row in 1.. {
        let Some(cell) = worksheet.get_cell((1, row)) else {
            break;
        };

        let username = cell.get_cell_value().get_value().to_string();

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

        let guess = worksheet.get_value((2, row));

        contents.push((row, username, color, guess));
    }

    contents
}
