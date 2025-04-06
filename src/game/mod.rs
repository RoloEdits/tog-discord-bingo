use std::str::FromStr;
use std::time::Duration;
use std::{fmt::Write as _, fs::File, io::Write as _, path::Path};

use headless_chrome::{Browser, protocol::cdp::Page::CaptureScreenshotFormatOption, types::Bounds};

use crate::error::Error;
use crate::spreadsheet::Row;
use crate::{Guess, Key, Player, Square};

pub mod great_war;
pub mod normal;

#[macro_export]
macro_rules! board {
    [ $( [ $( $x:expr ),* $(,)? ] ),* $(,)? ] => {{
        const DATA: &[i32] = &[
            $(
                $( $x ),*
            ),*
        ];
        DATA
    }};
}

pub trait Game {
    // TODO: Block submitting if key size that does not match total squares.
    const SQUARES: usize;

    const BOARD: &[i32];

    // Can fail if guessers have incorrect number of guesses
    fn from_rows(rows: &[Row]) -> Result<Self, Error>
    where
        Self: std::marker::Sized;

    fn players(&self) -> &[Player];

    fn players_mut(&mut self) -> &mut [Player];

    fn play(&mut self, key: &Key) {
        for player in self.players_mut() {
            for (square, (guess, key)) in player.guess.iter().zip(key).enumerate() {
                if guess == &Square::Pass {
                    continue;
                }

                if guess == key {
                    player.score += Self::BOARD[square];
                } else {
                    player.score -= Self::BOARD[square];
                }
            }
        }

        // Sort by score, and when the scores match, by name.
        self.players_mut()
            .sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
    }

    fn players_from_rows(rows: &[Row]) -> Result<Vec<Player>, Error> {
        let mut players = Vec::with_capacity(rows.len());

        for row in rows {
            let Ok(guess) = Guess::from_str(row.guess());

            // if guess.len() != Self::SQUARES {
            //     return Err(Error::NotEnoughValidSquares {
            //         name: row.name().text().to_string(),
            //         row: row.num(),
            //         amount: guess.len(),
            //         needed: Self::SQUARES,
            //     });
            // }

            let player = Player {
                name: row.name().text().to_string(),
                color: row.name().color().to_hex(),
                guess,
                score: row.starting_score(),
            };

            // NOTE: Could use a `HashSet`, but given the small number of players it shouldn't matter.
            // `HashSet::insert` also takes ownership and drops the inserted value if it already exists.
            // For nicer error messages, its more convenient to be able to make a `Vec` of the double guessers use that
            // to inform the user everyone who offended.
            //
            // Another option could be to just silently replace with new guess, or even exclude them altogether.
            // if players.contains(&player) {
            //     return Err(Error::DoubleGuesser { row: row.num(), name: row.name().text().to_string() });
            // }

            players.push(player);
        }

        Ok(players)
    }

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
                    write!(html,"<tr class=\" border-b border-zinc-700\">
                                                <th scope=\"row\" class=\"px-6 py-4 font-medium whitespace-nowrap\" style=\"color: {};\">", player.color ).unwrap();

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

                let browser = Browser::new(headless_chrome::LaunchOptions {idle_browser_timeout: Duration::from_secs(600),..Default::default()}).unwrap();

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
                    width: Some(1_000.0),
                    height: Some(15_000.0),
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
