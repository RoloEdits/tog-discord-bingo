use std::path::Path;

use crate::{error::Error, Guess, Key, Player, Square};

use super::Algorithm;

#[derive(Debug)]
pub struct GreatWar {
    players: Vec<Player>,
}

impl Algorithm for GreatWar {
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

            if guess.len() < 78 {
                return Err(Error::NotEnoughValidSquares {
                    name,
                    row,
                    amount: guess.len(),
                    needed: 78,
                });
            }

            let mut score: i32 = 0;

            score += worksheet.get_value((3, row)).parse::<i32>().unwrap();
            score += worksheet.get_value((4, row)).parse::<i32>().unwrap();
            score += worksheet.get_value((5, row)).parse::<i32>().unwrap();

            let player = Player {
                row,
                name,
                color,
                guess,
                score,
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
        for player in &mut self.players {
            player.score += score(key.as_squares(), player.guess.as_squares());

            debug_assert!(player.score != 0, "player score cannot equal zero");

            debug_assert!(
                (-3440..=3440).contains(&player.score),
                "player score can only be from -3440 to 34400 but was {}",
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

const GREEN: i32 = 15;
const YELLOW: i32 = 30;
const RED: i32 = 50;
const PURPLE: i32 = 70;

fn score(key: &[Square], guess: &[Square]) -> i32 {
    let mut score = 0;

    score += score_row_one(key, guess);
    score += score_row_two(key, guess);
    score += score_row_three(key, guess);
    score += score_row_four(key, guess);
    score += score_row_five(key, guess);
    score += score_row_six(key, guess);
    score += score_row_seven(key, guess);
    score += score_row_eight(key, guess);
    score += score_bonus(key, guess);

    score
}

fn score_row_one(key: &[Square], guess: &[Square]) -> i32 {
    const A1: usize = 0;
    const B1: usize = 1;
    const C1: usize = 2;
    const D1: usize = 3;
    const E1: usize = 4;
    const F1: usize = 5;
    const G1: usize = 6;
    const H1: usize = 7;

    let mut score = 0;

    if guess[A1] == key[A1] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[B1] == key[B1] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[C1] == key[C1] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[D1] == key[D1] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[E1] == key[E1] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[F1] == key[F1] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[G1] == key[G1] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[H1] == key[H1] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    score
}

fn score_row_two(key: &[Square], guess: &[Square]) -> i32 {
    const A2: usize = 8;
    const B2: usize = 9;
    const C2: usize = 10;
    const D2: usize = 11;
    const E2: usize = 12;
    const F2: usize = 13;
    const G2: usize = 14;
    const H2: usize = 15;
    let mut score = 0;

    if guess[A2] == key[A2] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[B2] == key[B2] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[C2] == key[C2] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[D2] == key[D2] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[E2] == key[E2] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[F2] == key[F2] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[G2] == key[G2] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[H2] == key[H2] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    score
}
fn score_row_three(key: &[Square], guess: &[Square]) -> i32 {
    const A3: usize = 16;
    const B3: usize = 17;
    const C3: usize = 18;
    const D3: usize = 19;
    const E3: usize = 20;
    const F3: usize = 21;
    const G3: usize = 22;
    const H3: usize = 23;

    let mut score = 0;

    if guess[A3] == key[A3] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[B3] == key[B3] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[C3] == key[C3] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[D3] == key[D3] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[E3] == key[E3] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[F3] == key[F3] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[G3] == key[G3] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[H3] == key[H3] {
        score += RED;
    } else {
        score -= RED;
    }

    score
}
fn score_row_four(key: &[Square], guess: &[Square]) -> i32 {
    const A4: usize = 24;
    const B4: usize = 25;
    const C4: usize = 26;
    const D4: usize = 27;
    const E4: usize = 28;
    const F4: usize = 29;
    const G4: usize = 30;
    const H4: usize = 31;

    let mut score = 0;

    if guess[A4] == key[A4] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[B4] == key[B4] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[C4] == key[C4] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[D4] == key[D4] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[E4] == key[E4] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[F4] == key[F4] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[G4] == key[G4] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[H4] == key[H4] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    score
}
fn score_row_five(key: &[Square], guess: &[Square]) -> i32 {
    const A5: usize = 32;
    const B5: usize = 33;
    const C5: usize = 34;
    const D5: usize = 35;
    const E5: usize = 36;
    const F5: usize = 37;
    const G5: usize = 38;
    const H5: usize = 39;

    let mut score = 0;

    if guess[A5] == key[A5] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[B5] == key[B5] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[C5] == key[C5] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[D5] == key[D5] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[E5] == key[E5] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[F5] == key[F5] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[G5] == key[G5] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[H5] == key[H5] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    score
}
fn score_row_six(key: &[Square], guess: &[Square]) -> i32 {
    const A6: usize = 40;
    const B6: usize = 41;
    const C6: usize = 42;
    const D6: usize = 43;
    const E6: usize = 44;
    const F6: usize = 45;
    const G6: usize = 46;
    const H6: usize = 47;

    let mut score = 0;

    if guess[A6] == key[A6] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[B6] == key[B6] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[C6] == key[C6] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[D6] == key[D6] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[E6] == key[E6] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[F6] == key[F6] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[G6] == key[G6] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[H6] == key[H6] {
        score += RED;
    } else {
        score -= RED;
    }

    score
}

fn score_row_seven(key: &[Square], guess: &[Square]) -> i32 {
    const A7: usize = 48;
    const B7: usize = 49;
    const C7: usize = 50;
    const D7: usize = 51;
    const E7: usize = 52;
    const F7: usize = 53;
    const G7: usize = 54;
    const H7: usize = 55;

    let mut score = 0;

    if guess[A7] == key[A7] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[B7] == key[B7] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[C7] == key[C7] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[D7] == key[D7] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[E7] == key[E7] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[F7] == key[F7] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[G7] == key[G7] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[H7] == key[H7] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    score
}

fn score_row_eight(key: &[Square], guess: &[Square]) -> i32 {
    const A8: usize = 56;
    const B8: usize = 57;
    const C8: usize = 58;
    const D8: usize = 59;
    const E8: usize = 60;
    const F8: usize = 61;
    const G8: usize = 62;
    const H8: usize = 63;

    let mut score = 0;

    if guess[A8] == key[A8] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[B8] == key[B8] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[C8] == key[C8] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[D8] == key[D8] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[E8] == key[E8] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    if guess[F8] == key[F8] {
        score += YELLOW;
    } else {
        score -= YELLOW;
    }

    if guess[G8] == key[G8] {
        score += RED;
    } else {
        score -= RED;
    }

    if guess[H8] == key[H8] {
        score += GREEN;
    } else {
        score -= GREEN;
    }

    score
}

fn score_bonus(key: &[Square], guess: &[Square]) -> i32 {
    const A9: usize = 64;
    const A10: usize = 71;
    const B9: usize = 65;
    const B10: usize = 72;
    const C9: usize = 66;
    const C10: usize = 73;
    const D9: usize = 67;
    const D10: usize = 74;
    const E9: usize = 68;
    const E10: usize = 75;
    const F9: usize = 69;
    const F10: usize = 76;
    const G9: usize = 70;
    const G10: usize = 77;

    let mut score = 0;

    if guess[A9] == key[A9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[B9] == key[B9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[C9] == key[C9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[D9] == key[D9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[E9] == key[E9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[F9] == key[F9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[G9] == key[G9] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[A10] == key[A10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[B10] == key[B10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[C10] == key[C10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[D10] == key[D10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[E10] == key[E10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[F10] == key[F10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    if guess[G10] == key[G10] {
        score += PURPLE;
    } else {
        score -= PURPLE;
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    const GUESS: &str = r"
Y Y N Y N N N Y
Y Y Y N Y Y Y N
Y N Y N N Y Y N
N Y Y Y Y N N Y
N Y N Y Y N Y Y
Y N Y Y N Y Y N
N Y N Y Y Y Y N
Y Y Y N Y N Y Y

Y Y Y Y N Y Y
Y N N N N N N";

    const KEY: &str = r"
Y Y N Y N N N Y
Y Y Y N Y Y Y N
Y N Y N N Y Y N
N Y Y Y Y N N Y
N Y N Y Y N Y Y
Y N Y Y N Y Y N
N Y N Y Y Y Y N
Y Y Y N Y N Y Y

Y Y Y Y N Y Y
Y N N N N N N";

    #[test]
    fn should_get_full_score() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);

        let score = score(key.as_squares(), guess.as_squares());

        assert_eq!(2990, score);
    }

    #[test]
    fn should_score_row_one() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_one(key.as_squares(), guess.as_squares());

        assert_eq!(235, score);
    }

    #[test]
    fn should_score_row_two() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_two(key.as_squares(), guess.as_squares());

        assert_eq!(255, score);
    }

    #[test]
    fn should_score_row_three() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_three(key.as_squares(), guess.as_squares());

        assert_eq!(270, score);
    }
    #[test]
    fn should_score_row_four() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_four(key.as_squares(), guess.as_squares());

        assert_eq!(235, score);
    }

    #[test]
    fn should_score_row_five() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_five(key.as_squares(), guess.as_squares());

        assert_eq!(255, score);
    }

    #[test]
    fn should_score_row_six() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_six(key.as_squares(), guess.as_squares());

        assert_eq!(270, score);
    }

    #[test]
    fn should_score_row_seven() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_seven(key.as_squares(), guess.as_squares());

        assert_eq!(235, score);
    }

    #[test]
    fn should_score_row_eight() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_row_eight(key.as_squares(), guess.as_squares());

        assert_eq!(255, score);
    }

    #[test]
    fn should_score_row_bonus() {
        let key = Key::parse(KEY);
        let guess = Guess::parse(GUESS);
        let score = score_bonus(key.as_squares(), guess.as_squares());

        assert_eq!(980, score);
    }
}
