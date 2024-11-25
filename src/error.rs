#[derive(Debug)]
pub enum Error {
    DoubleGuesser {
        row: u32,
    },
    NotEnoughValidSquares {
        name: String,
        row: u32,
        amount: usize,
        needed: usize,
    },
}
