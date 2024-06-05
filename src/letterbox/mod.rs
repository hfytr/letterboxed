mod builder;
mod solver;
use std::char;

#[derive(Default)]
pub struct LetterBoxed {
    letters: Vec<Vec<char>>,
}

impl LetterBoxed {
    pub fn new(letters: Vec<Vec<char>>) -> LetterBoxed {
        Self { letters }
    }
}
