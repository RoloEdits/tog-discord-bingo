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
        // NOTE: hex code includes alpha prefix. Just want the last 6 digits.
        let hex = cell.get_style().get_font().unwrap().get_color().get_argb();

        let color = if hex.is_empty() {
            // Default to white
            String::from("#FFFFFF")
        } else {
            format!("#{}", &hex[2..])
        };

        let guess = worksheet.get_value((2, row));

        contents.push((row, username, color, guess));
    }

    contents
}
