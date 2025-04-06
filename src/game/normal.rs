use crate::{Player, board, error::Error, game::Game, spreadsheet::Row};

#[derive(Debug)]
pub struct Normal {
    players: Vec<Player>,
}

impl Game for Normal {
    const SQUARES: usize = 12;

    const BOARD: &[i32] = board![
        [10, 10, 10, 20], //
        [30, 30, 30, 60],
        [50, 50, 50, 100],
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
mod tests {
    use super::*;
    use crate::{Key, spreadsheet::Name};
    use eframe::egui::Color32;
    use std::str::FromStr;

    #[test]
    fn should_score_450() {
        let mut game = Normal::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess: String::from("YYYY YYYY YYYY"),
            starting_score: 0,
        }])
        .unwrap();

        let key = Key::from_str("YYYY YYYY YYYY").unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(450, player.score);
    }

    #[test]
    fn should_score_neg_450() {
        let mut game = Normal::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess: String::from("NNNN NNNN NNNN"),
            starting_score: 0,
        }])
        .unwrap();

        let key = Key::from_str("YYYY YYYY YYYY").unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(-450, player.score);
    }

    #[test]
    fn should_score_with_passes() {
        let mut game = Normal::from_rows(&[Row {
            num: 1,
            name: Name::new(String::from("Rolo"), Color32::from_rgb(0, 0, 0)),
            guess: String::from("YYYP YYYP YYYP"),
            starting_score: 0,
        }])
        .unwrap();

        let key = Key::from_str("YYYY YYYY YYYY").unwrap();

        game.play(&key);

        let player = &game.players[0];

        assert_eq!(270, player.score);
    }
}
