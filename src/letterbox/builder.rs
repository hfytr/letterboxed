use crate::letterbox::LetterBoxed;
use crate::trie::Trie;
use top_english_words::get_words;

impl LetterBoxed {
    pub fn build_puzzle_trie(&self) -> Trie {
        let english_trie = self.build_english_trie();
        let mut trie = Trie::new();
        for i in 0..self.letters.len() {
            self.build_puzzle_trie_helper(i as u8, String::new(), &mut trie, &english_trie);
        }
        trie
    }

    fn build_puzzle_trie_helper(
        &self,
        side: u8,
        prefix: String,
        puzzle_trie: &mut Trie,
        english_trie: &Trie,
    ) {
        let mut next_prefixes: Vec<(u8, String)> = Vec::new();
        for (cur_side, side_letters) in self.letters.iter().enumerate() {
            if cur_side as u8 == side {
                continue;
            }
            let valid_next_prefixes = side_letters
                .iter()
                .map(|c| {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(*c);
                    new_prefix
                })
                .filter(|s| english_trie.is_node(s))
                .map(|s| (cur_side as u8, s));
            for a in valid_next_prefixes {
                next_prefixes.push(a);
            }
        }
        for (child_side, child_prefix) in next_prefixes {
            if english_trie.is_key(&child_prefix) && child_prefix.len() > 1 {
                puzzle_trie.insert(&child_prefix);
            }
            self.build_puzzle_trie_helper(child_side, child_prefix, puzzle_trie, english_trie)
        }
    }

    fn build_english_trie(&self) -> Trie {
        Trie::from_vec(get_words::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_trie() {
        let lb = LetterBoxed::new(vec![vec!['o', 'l'], vec!['a', 'r'], vec!['p', 'd']]);
        let trie = lb.build_puzzle_trie();
        let trie_vec: Vec<String> = trie.list_members().unwrap();
        println!("{:?}", trie_vec);
    }
}
