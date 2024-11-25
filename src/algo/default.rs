use std::path::Path;

use crate::{error::Error, Guess, Key, Player, Square};

use super::Algorithm;

#[derive(Debug)]
pub struct Default {
    players: Vec<Player>,
}

impl Algorithm for Default {
    fn read(path: &Path) -> Result<Self, Error> {
        let mut players = Vec::new();

        let workbook = umya_spreadsheet::reader::xlsx::read(path).unwrap();

        let worksheet = workbook
            .get_sheet_by_name("Sheet1")
            .expect("failed to read worksheet");

        for row in 1.. {
            let Some(cell) = worksheet.get_cell((1, row)) else {
                break;
            };

            let name = cell.get_cell_value().get_value().to_string();

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

            let guess = Guess::parse(&worksheet.get_value((2, row)));

            if guess.len() < 12 {
                return Err(Error::NotEnoughValidSquares {
                    name,
                    row,
                    amount: guess.len(),
                    needed: 12,
                });
            }

            let player = Player {
                row,
                name,
                color,
                guess,
                score: 0,
            };

            // NOTE: Could use a `HashSet`, but given the small number of players it shouldn't matter.
            // `HashSet::insert` also takes ownership and drops the inserted value if it already exists.
            // For nicer error messages, its more convenient to be able to make a `Vec` of the double guessers use that
            // to inform the user everyone who offended.
            //
            // Another option could be to just silently replace with new guess, or even exclude them altogether.
            if players.contains(&player) {
                return Err(Error::DoubleGuesser { row });
            }
            players.push(player);
        }

        Ok(Self { players })
    }

    fn score(&mut self, key: &Key) -> &[Player] {
        const ROWS: [i32; 3] = [10, 30, 50];

        for player in &mut self.players {
            let mut row = 0;

            for (idx, (guess, key)) in player.guess.iter().zip(key).enumerate() {
                if matches!(idx, 4 | 8) {
                    row += 1;
                }

                if guess == &Square::Pass {
                    continue;
                }

                // Bonus
                if matches!(idx, 3 | 7 | 11) {
                    if guess == key {
                        player.score += ROWS[row] * 2;
                    } else {
                        player.score -= ROWS[row] * 2;
                    }
                } else if guess == key {
                    player.score += ROWS[row];
                } else {
                    player.score -= ROWS[row];
                }
            }

            debug_assert!(player.score != 0, "player score cannot equal zero");

            debug_assert!(
                (-450..=450).contains(&player.score),
                "player score can only be from -450 to 450 but was {}",
                player.score
            );
        }

        // Sort by score and when the scores match, by name.
        self.players
            .sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));

        &self.players
    }

    fn players(&self) -> &[crate::Player] {
        &self.players
    }
}
