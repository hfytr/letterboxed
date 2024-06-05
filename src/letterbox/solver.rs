use super::LetterBoxed;

impl LetterBoxed {
    pub fn solve(&mut self) -> Vec<Vec<char>> {
        let puzzle_trie = self.build_puzzle_trie();
    }
}
