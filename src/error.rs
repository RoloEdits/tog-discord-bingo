#[derive(Debug)]
pub enum Error {
    DoubleGuesser {
        row: u32,
        name: String,
    },
    NotEnoughValidSquares {
        name: String,
        row: u32,
        amount: usize,
        needed: usize,
    },
}
