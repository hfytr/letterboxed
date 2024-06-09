use super::LetterBoxed;
use crate::trie::get_let_num;
use std::cmp::min;

pub fn post_process(v: Vec<String>) -> Vec<String> {
    v.into_iter()
        .map(|s| s.chars().rev().collect::<String>())
        .rev()
        .collect()
}

impl LetterBoxed {
    pub fn solve(&mut self) -> Vec<String> {
        let mut best = Vec::new();
        let mut first = true;

        for letter in self.letters.concat() {
            let start_index = self.index_from_letter[get_let_num(letter as u8)];
            let search_result = self.helper(start_index, std::u8::MAX);

            if search_result.is_none() {
                continue;
            }
            let cur = search_result.unwrap();
            if cur.len() < best.len() || first {
                best = cur;
                first = false;
            }
        }
        post_process(best)
    }

    pub fn helper(&mut self, last_ind: (usize, usize), to_beat: u8) -> Option<Vec<String>> {
        let vis = self.vis[last_ind.0][last_ind.1].last();
        if (vis.is_some() && *vis.unwrap() == self.num_used_letters) || to_beat == 0 {
            return None;
        }
        if self.num_used_letters == self.total_letters {
            return Some(Vec::new());
        }

        let c = self.letters[last_ind.0][last_ind.1];
        let mut keys = self.puzzle_trie.list_reversed_keys(&c.to_string()).unwrap();
        let children = keys.iter_mut().filter(|s| s.len() > 1).map(|s| {
            s.push(c);
            s
        });
        let mut best = Vec::new();
        let mut cur;
        let mut first = true;

        for child in children {
            // child is reversed so first is last
            let child_pos = self.index_from_letter[get_let_num(child.as_bytes()[0])];
            let child_to_beat = if best.is_empty() {
                to_beat - 1
            } else {
                min(best.len() as u8, to_beat) - 1
            };

            self.update(child, child_pos);
            let search_result = self.helper(child_pos, child_to_beat);
            self.undo(child, child_pos);

            if search_result.is_none() {
                continue;
            }

            cur = search_result.unwrap();
            cur.push(child.to_string());

            if cur.len() < best.len() || first {
                best = cur;
                first = false;
                if best.len() == 1 {
                    break;
                }
            }
        }
        if first {
            None
        } else {
            Some(best)
        }
    }

    fn update(&mut self, s: &str, last_pos: (usize, usize)) {
        self.vis[last_pos.0][last_pos.1].push(self.num_used_letters);
        for c in s.as_bytes() {
            let index = self.index_from_letter[get_let_num(*c)];
            if self.used_letters[index.0][index.1] == 0 {
                self.num_used_letters += 1;
            }
            self.used_letters[index.0][index.1] += 1;
        }
    }

    fn undo(&mut self, s: &str, last_pos: (usize, usize)) {
        for c in s.as_bytes() {
            let index = self.index_from_letter[get_let_num(*c)];
            // if this becomes < 0 you fucked up
            self.used_letters[index.0][index.1] -= 1;
            if self.used_letters[index.0][index.1] == 0 {
                self.num_used_letters -= 1;
            }
        }
        self.vis[last_pos.0][last_pos.1].pop();
    }
}
