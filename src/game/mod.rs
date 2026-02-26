use std::fmt::Write;
use std::path::Path;
use std::str::FromStr;

use resvg::tiny_skia::Pixmap;
use resvg::usvg::{Options, Tree};

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

    fn save_png(&self, path: &Path) {
        let path = path.with_extension("png");
        let svg = svg(self.players());

        std::thread::spawn(move || {
            let mut opt = Options::default();

            opt.fontdb_mut()
                .load_font_data(super::fonts::GGSANS.to_vec());
            opt.fontdb_mut()
                .load_font_data(super::fonts::SEGUIHIS.to_vec());
            opt.fontdb_mut()
                .load_font_data(super::fonts::SEGUISYM.to_vec());
            opt.fontdb_mut()
                .load_font_data(super::fonts::NOTO_SANS_CHINENSE.to_vec());
            opt.fontdb_mut()
                .load_font_data(super::fonts::NOTO_SANS_KOREAN.to_vec());

            let tree = Tree::from_str(&svg, &opt).expect("Invalid SVG");

            let size = tree.size();

            let mut pixelmap = Pixmap::new(size.width() as u32, size.height() as u32)
                .expect("Faild to create Pixmap");

            resvg::render(
                &tree,
                resvg::usvg::Transform::default(),
                &mut pixelmap.as_mut(),
            );

            pixelmap.save_png(path).expect("Failed to save PNG");
        });
    }
}

#[must_use]
pub fn svg(players: &[Player]) -> String {
    let row_height = 70;

    let px = 20;
    let padding = 50;

    let name = players
        .iter()
        .map(|p| p.name.chars().count())
        .max()
        .unwrap_or(10);

    let x = 50 + name as u32 * px + padding;

    let width = x + 150;
    let height = row_height * players.len() as u32;

    let mut svg = String::new();

    let bg = "#222226";

    write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">
        <style>
            .body-bg {{ fill: {bg}; }}
            .body-text {{ font-family: gg sans,Noto Sans SC,Segoe UI Historic,Segoe UI Symbol; font-size: 36px; }}
        </style>

        <rect class="body-bg" width="100%" height="100%"/>
    "#
    )
    .unwrap();

    for (idx, player) in players.iter().enumerate() {
        let y = idx as u32 * row_height;

        write!(
            svg,
            r##"
            <text class="body-text" x="{padding}" y="{}" fill="{}">{}</text>
            <text class="body-text" x="{x}" y="{}" fill="#e6e6e8" text-anchor="end">{}</text>
            "##,
            y + 45,
            player.color,
            player.name,
            y + 45,
            player.score
        )
        .unwrap();
    }

    svg.push_str("</svg>");

    svg
}
