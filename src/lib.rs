use error::Error;
use std::{fmt::Display, hash::Hash, path::Path};

pub mod error;

mod algo;
use algo::{default::Default, great_war::GreatWar, Algorithm};

#[derive(Debug)]
pub enum Bingo {
    Default(Default),
    GreatWar(GreatWar),
}

impl Bingo {
    pub fn default(path: &Path) -> Result<Self, Error> {
        Ok(Self::Default(Default::read(path)?))
    }

    pub fn great_war(path: &Path) -> Result<Self, Error> {
        Ok(Self::GreatWar(GreatWar::read(path)?))
    }

    pub fn players(&self) -> &[Player] {
        match self {
            Self::Default(default) => default.players(),
            Self::GreatWar(great_war) => great_war.players(),
        }
    }

    pub fn score(&mut self, key: &Key) -> &[Player] {
        match self {
            Bingo::Default(default) => default.score(key),
            Self::GreatWar(great_war) => great_war.score(key),
        }
    }

    pub fn save_html(&self, path: &Path) {
        match self {
            Bingo::Default(default) => default.save_html(path),
            Self::GreatWar(great_war) => great_war.save_html(path),
        }
    }
}

#[derive(Debug)]
pub struct Key(Vec<Square>);

impl Key {
    pub fn parse(key: &str) -> Self {
        let mut result: Vec<Square> = Vec::with_capacity(12);

        for square in key.chars() {
            if matches!(square, 'Y' | 'y' | 'N' | 'n' | 'P' | 'p') {
                result.push(Square::from_char(square));
            }
        }

        Self(result)
    }

    fn as_squares(&self) -> &[Square] {
        &self.0
    }

    #[allow(dead_code)]
    fn iter(&self) -> std::slice::Iter<'_, Square> {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl<'a> IntoIterator for &'a Key {
    type Item = &'a Square;
    type IntoIter = std::slice::Iter<'a, Square>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(PartialOrd, Ord, Debug, Clone)]
pub struct Player {
    pub row: u32,
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
    fn parse(s: &str) -> Self {
        let mut guess: Vec<Square> = Vec::new();

        for square in s.chars() {
            if matches!(square, 'Y' | 'y' | 'N' | 'n' | 'P' | 'p') {
                guess.push(Square::from_char(square));
            }
        }
        Self(guess)
    }

    fn as_squares(&self) -> &[Square] {
        &self.0
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> std::slice::Iter<'_, Square> {
        <&Self as IntoIterator>::into_iter(self)
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
