use error::Error;
use std::{convert::Infallible, fmt::Display, hash::Hash, path::Path, str::FromStr};

pub mod error;
pub mod game;
pub mod spreadsheet;

use game::{Game, great_war::GreatWar, normal::Normal};
use spreadsheet::Row;

#[derive(Debug)]
pub enum Bingo {
    Normal(Normal),
    GreatWar(GreatWar),
}

impl Bingo {
    pub fn normal(rows: &[Row]) -> Result<Self, Error> {
        Ok(Self::Normal(Normal::from_rows(rows)?))
    }

    pub fn great_war(rows: &[Row]) -> Result<Self, Error> {
        Ok(Self::GreatWar(GreatWar::from_rows(rows)?))
    }

    pub fn players(&self) -> &[Player] {
        match self {
            Self::Normal(normal) => normal.players(),
            Self::GreatWar(great_war) => great_war.players(),
        }
    }

    pub fn play(&mut self, key: &Key) {
        match self {
            Self::Normal(normal) => normal.play(key),
            Self::GreatWar(great_war) => great_war.play(key),
        }
    }

    pub fn save_html(&self, path: &Path) {
        match self {
            Self::Normal(normal) => normal.save_html(path),
            Self::GreatWar(great_war) => great_war.save_html(path),
        }
    }
}

#[derive(Debug)]
pub struct Key(Vec<Square>);

impl FromStr for Key {
    type Err = Infallible;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        let mut result: Vec<Square> = Vec::with_capacity(12);

        for square in key.chars() {
            if matches!(square, 'Y' | 'y' | 'N' | 'n') {
                result.push(Square::from_char(square));
            }
        }

        Ok(Self(result))
    }
}

impl<'a> IntoIterator for &'a Key {
    type Item = &'a Square;
    type IntoIter = std::slice::Iter<'a, Square>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_slice().iter()
    }
}

#[derive(PartialOrd, Ord, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub color: String,
    pub guess: Guess,
    pub score: i32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Player {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Guess(Vec<Square>);

impl Guess {
    fn iter(&self) -> std::slice::Iter<'_, Square> {
        self.0.as_slice().iter()
    }
}

impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for square in &self.0 {
            write!(f, "{square}")?;
        }

        Ok(())
    }
}

impl FromStr for Guess {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guess: Vec<Square> = Vec::with_capacity(s.len());

        for square in s.chars() {
            if square.is_whitespace() {
                continue;
            }

            if matches!(square, 'Y' | 'y' | 'N' | 'n' | 'P' | 'p') {
                guess.push(Square::from_char(square));
            }
        }
        Ok(Self(guess))
    }
}

impl<'a> IntoIterator for &'a Guess {
    type Item = &'a Square;
    type IntoIter = std::slice::Iter<'a, Square>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    Yes,
    No,
    Pass,
}

impl Square {
    fn from_char(guess: char) -> Self {
        match guess {
            'Y' | 'y' => Self::Yes,
            'N' | 'n' => Self::No,
            'P' | 'p' => Self::Pass,
            unknown => {
                panic!("square guess was `{unknown}`, must be one of `Y`, `y`, `N`, `n`, `P`, `p`")
            }
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            Self::Yes => "Y",
            Self::No => "N",
            Self::Pass => "P",
        };

        write!(f, "{letter}")?;

        Ok(())
    }
}
