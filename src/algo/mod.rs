use std::{fs::File, io::Write, path::Path};

use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, types::Bounds, Browser};

use crate::{error::Error, Key, Player};

pub mod default;
pub mod great_war;

pub trait Algorithm {
    // Required
    fn read(path: &Path) -> Result<Self, Error>
    where
        Self: std::marker::Sized;

    fn score(&mut self, key: &Key) -> &[Player];

    fn players(&self) -> &[Player];

    // Provided
    fn save_html(&self, path: &Path) {
        let players = self.players();
        std::thread::scope(|s| {
            s.spawn(|| {
                let mut html = String::new();

                html.push_str(
                    r#"<!DOCTYPE html>
            <html>

            <head>
                <meta charset="UTF-8" />
                <title>title</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>

            <body class="max-w-md">
                <div class="relative">
                    <table class="w-full text-4xl text-left text-zinc-50">
                        <thead class="text-xl uppercase bg-zinc-700 text-zinc-50">
                            <tr>
                                <th scope="col" class="px-6 py-3">
                                    Name
                                </th>
                                <th scope="col" class="px-6 py-3">
                                    Score
                                </th>
                            </tr>
                        </thead>
                        <tbody class="bg-zinc-800 text-white">"#,
                );

                for player in players {
                    html.push_str(&format!("<tr class=\" border-b border-zinc-700\">
                                                <th scope=\"row\" class=\"px-6 py-4 font-medium whitespace-nowrap\" style=\"color: {};\">", player.color));

                    html.push_str(&player.name);

                    html.push_str(
                        r#"</th>
                        <td class="px-6 py-4 text-white">"#,
                    );

                    html.push_str(&player.score.to_string());

                    html.push_str(r"</td></tr>");
                }

                html.push_str(
                    "</tbody>
                    </table>
                </div>
            </body>
            </html>",
                );
                let dir = tempfile::tempdir().unwrap();

                let temp_path = dir
                    .path()
                    .with_file_name(path.file_name().unwrap())
                    .with_extension("html");

                let mut temp = File::create(&temp_path).unwrap();

                temp.write_all(html.as_bytes()).unwrap();

                let browser = Browser::default().unwrap();

                let tab = browser.new_tab().unwrap();

                tab.navigate_to(temp_path.as_os_str().to_str().unwrap())
                    .unwrap()
                    .wait_until_navigated()
                    .unwrap();

                let element = tab.find_element("tbody").unwrap();

                tab.set_bounds(Bounds::Normal {
                    left: Some(0),
                    top: Some(0),
                    // table width + right vertical scroll bar
                    width: Some(5_000.0),
                    height: Some(100_000.0),
                })
                .unwrap();

                let bytes = element
                    .capture_screenshot(CaptureScreenshotFormatOption::Png)
                    .unwrap();

                let image =
                    image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).unwrap();

                let cropped = image.crop_imm(0, 0, image.width() - 17, image.height());
                cropped.save(path.with_extension("png")).unwrap();
            });
        });
    }
}
