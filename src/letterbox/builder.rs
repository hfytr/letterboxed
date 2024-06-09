use crate::letterbox::LetterBoxed;
use crate::trie::{get_let_num, Trie};
use crate::words::get_words;

impl LetterBoxed {
    pub fn new(letters: Vec<Vec<char>>) -> LetterBoxed {
        let mut index_from_letter = [(0_usize, 0_usize); 26];
        let mut side = 0_usize;
        let mut i = 0_usize;

        for letter in letters.concat() {
            if i == letters[side].len() {
                i = 0;
                side += 1;
            }
            index_from_letter[get_let_num(letter as u8)] = (side, i);
            i += 1;
        }

        let mut lb = Self {
            used_letters: letters.iter().map(|v| vec![0; v.len()]).collect(),
            num_used_letters: 0,
            index_from_letter,
            total_letters: letters.iter().fold(0, |acc, v| acc + v.len() as u8),
            vis: letters.iter().map(|v| vec![Vec::new(); v.len()]).collect(),
            letters,
            english_trie: Trie::default(),
            puzzle_trie: Trie::default(),
        };
        lb.build_puzzle_trie();
        lb
    }

    pub fn build_puzzle_trie(&mut self) {
        let english_trie = Self::build_english_trie();
        let mut trie = Trie::new();
        for i in 0..self.letters.len() {
            self.build_puzzle_trie_helper(i as u8, String::new(), &mut trie, &english_trie);
        }
        self.puzzle_trie = trie
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

    fn build_english_trie() -> Trie {
        Trie::from_vec(get_words())
    }
}
