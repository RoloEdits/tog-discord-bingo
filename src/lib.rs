use arrayvec::ArrayVec;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, types::Bounds, Browser};
use std::{fmt::Display, fs::File, hash::Hash, io::Write, path::PathBuf};

pub enum BingoError {
    DoubleGuesser {
        name: String,
        row: u32,
        guess: String,
    },
}

#[derive(Debug, Default)]
pub struct Bingo {
    pub players: Vec<Player>,
}

impl Bingo {
    pub fn add_player(&mut self, player: Player) -> Result<(), BingoError> {
        // NOTE: Could use a `HashSet`, but given the small number of players it shouldnt matter.
        // `HashSet::insert` also takes ownership and drops the inserted value if it already exists.
        // For nicer error messages, its more convenient to be able to make a `Vec` of the double guessers use that
        // to inform the user everyone who offended.
        //
        // Another option could be to just silently replace with new guess, or even exclude them altogether.
        if self.players.contains(&player) {
            let found = self
                .players
                .iter()
                .find(|offender| **offender == player)
                .expect("Vec contains player ");

            return Err(BingoError::DoubleGuesser {
                name: found.name.clone(),
                row: found.row,
                guess: found.guess.to_string(),
            });
        }

        self.players.push(player);

        Ok(())
    }

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    const ROWS: [i16; 3] = [10, 30, 50];

    pub fn score(&mut self, key: &Key) -> &[Player] {
        for player in self.players.iter_mut() {
            let mut row = 0;

            for (idx, (guess, key)) in player.guess.iter().zip(key).enumerate() {
                if matches!(idx, 4 | 8) {
                    row += 1;
                }

                if guess == &Square::Pass {
                    continue;
                }

                // bonus
                if matches!(idx, 3 | 7 | 11) {
                    if guess == key {
                        player.score += Self::ROWS[row] * 2;
                    } else {
                        player.score -= Self::ROWS[row] * 2;
                    }
                } else if guess == key {
                    player.score += Self::ROWS[row];
                } else {
                    player.score -= Self::ROWS[row];
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

    pub fn save_html(&self, path: PathBuf) {
        let players = self.players.clone();

        std::thread::spawn(move || {
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

            let height = element.get_box_model().unwrap().height;
            let width = element.get_box_model().unwrap().width;

            tab.set_bounds(Bounds::Normal {
                left: Some(0),
                top: Some(0),
                width: Some(width),
                height: Some(height),
            })
            .unwrap();

            let bytes = element
                .capture_screenshot(CaptureScreenshotFormatOption::Png)
                .unwrap();

            let image =
                image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).unwrap();

            let cropped = image.crop_imm(0, 0, image.width() - 17, image.height() - 16);

            cropped.save(path.with_extension("png")).unwrap();
        });
    }
}

pub enum KeyError {
    NotEnoughValidSquares,
}
#[derive(Debug)]
pub struct Key(ArrayVec<Square, 12>);

impl Key {
    pub fn new(key: &str) -> Result<Self, KeyError> {
        let mut result: ArrayVec<Square, 12> = ArrayVec::new();

        for square in key.chars() {
            if matches!(square, 'Y' | 'y' | 'N' | 'n' | 'P' | 'p') {
                result.push(Square::from_char(square));
            }
        }

        if result.len() != 12 {
            return Err(KeyError::NotEnoughValidSquares);
        }

        Ok(Self(result))
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

pub enum PlayerError {
    NotEnoughValidSquares,
}

#[derive(PartialOrd, Ord, Debug, Clone)]
pub struct Player {
    pub row: u32,
    pub name: String,
    pub color: String,
    pub guess: Guess,
    pub score: i16,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Player {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Guess(ArrayVec<Square, 12>);

impl Guess {
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

impl Player {
    pub fn new(row: u32, name: String, color: String, guess: String) -> Result<Self, PlayerError> {
        let mut guessess: ArrayVec<Square, 12> = ArrayVec::new();

        for square in guess.chars() {
            if matches!(square, 'Y' | 'y' | 'N' | 'n' | 'P' | 'p') {
                guessess.push(Square::from_char(square));
            }
        }

        drop(guess);

        if guessess.len() != 12 {
            return Err(PlayerError::NotEnoughValidSquares);
        }

        Ok(Self {
            row,
            name,
            color,
            guess: Guess(guessess),
            score: 0,
        })
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
