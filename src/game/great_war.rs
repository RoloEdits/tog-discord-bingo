use crate::{Player, board, error::Error, game::Game, spreadsheet::Row};

#[derive(Debug)]
pub struct GreatWar {
    players: Vec<Player>,
}

impl Game for GreatWar {
    const SQUARES: usize = 78;

    const BOARD: &[i32] = board![
        [15, 30, 50, 15, 30, 50, 15, 30],
        [50, 15, 30, 50, 15, 30, 50, 15],
        [30, 50, 15, 30, 50, 15, 30, 50],
        [15, 30, 50, 15, 30, 50, 15, 30],
        [50, 15, 30, 50, 15, 30, 50, 15],
        [30, 50, 15, 30, 50, 15, 30, 50],
        [15, 30, 50, 15, 30, 50, 15, 30],
        [50, 15, 30, 50, 15, 30, 50, 15],
        [70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70],
    ];

    fn from_rows(rows: &[Row]) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        const { assert!(Self::SQUARES == Self::BOARD.len()) }
        Ok(Self {
            players: Self::players_from_rows(rows)?,
        })
    }

    fn players(&self) -> &[Player] {
        &self.players
    }

    fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{Key, spreadsheet::Name};
    use eframe::egui::Color32;
    use std::str::FromStr;

    const GUESS: &str = "
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

    const KEY: &str = "
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
    fn should_score_2990() {
        let mut game = GreatWar::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess: String::from(GUESS),
            starting_score: 0,
        }])
        .unwrap();

        let key = Key::from_str(KEY).unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(2990, player.score);
    }

    #[test]
    fn should_score_neg_2990() {
        let guess = GUESS
            .chars()
            .filter(|char| !char.is_ascii_whitespace())
            .map(|char| if char == 'Y' { 'N' } else { 'Y' })
            .collect();

        let mut game = GreatWar::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess,
            starting_score: 0,
        }])
        .unwrap();

        let key = Key::from_str(KEY).unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(-2990, player.score);
    }

    #[test]
    fn should_score_max() {
        let mut game = GreatWar::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess: "Y".repeat(78),
            starting_score: 450,
        }])
        .unwrap();

        let key = Key::from_str(&"Y".repeat(78)).unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(3440, player.score);
    }
    #[test]
    fn should_score_min() {
        let mut game = GreatWar::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess: "N".repeat(78),
            starting_score: -450,
        }])
        .unwrap();

        let key = Key::from_str(&"Y".repeat(78)).unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(-3440, player.score);
    }
}
