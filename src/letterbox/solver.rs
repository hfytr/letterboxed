use crate::trie::get_let_num;

use super::LetterBoxed;

impl LetterBoxed {
    pub fn solve(&mut self) -> Vec<String> {
        let mut best = Vec::new();
        let mut cur;
        let mut first = true;
        for letter in self.letters.concat() {
            let start_index = self.index_from_letter[get_let_num(letter as u8)];
            cur = self.helper(start_index);
            if cur.len() < best.len() || first {
                best = cur;
            }
            first = false;
        }
        best
    }

    pub fn helper(&mut self, last_used: (usize, usize)) -> Vec<String> {
        if self.num_used_letters == self.total_letters {
            return Vec::new();
        }

        let children = self
            .puzzle_trie
            .list_keys(&self.letters[last_used.0][last_used.1].to_string())
            .unwrap();
        let mut best = Vec::new();
        let mut cur;
        let mut first = true;
        for child in children {
            self.update(&child);
            let child_last_used =
                self.index_from_letter[*child.as_bytes().last().unwrap() as usize];
            cur = self.helper(child_last_used);
            self.undo(&child);
            if cur.len() < best.len() || first {
                best = cur;
                best.push(child);
            }
            first = false;
        }
        best
    }

    fn update(&mut self, s: &str) {
        for c in s.as_bytes() {
            let index = self.index_from_letter[*c as usize];
            if self.used_letters[index.0][index.1] == 0 {
                self.num_used_letters += 1;
            }
            self.used_letters[index.0][index.1] += 1;
        }
    }

    fn undo(&mut self, s: &str) {
        for c in s.as_bytes() {
            let index = self.index_from_letter[*c as usize];
            // if this becomes < 0 you fucked up
            self.used_letters[index.0][index.1] -= 1;
            if self.used_letters[index.0][index.1] == 0 {
                self.num_used_letters -= 1;
            }
        }
    }
}
