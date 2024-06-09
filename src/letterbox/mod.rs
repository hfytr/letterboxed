use crate::trie::Trie;

mod builder;
mod solver;

#[derive(Default)]
pub struct LetterBoxed {
    letters: Vec<Vec<char>>,
    total_letters: u8,
    vis: Vec<Vec<Vec<u8>>>,
    used_letters: Vec<Vec<u8>>,
    num_used_letters: u8,
    index_from_letter: [(usize, usize); 26],
    english_trie: Trie,
    puzzle_trie: Trie,
}
