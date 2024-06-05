mod letterbox;
mod trie;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use self::letterbox::LetterBoxed;

    use super::*;

    #[test]
    fn smaller_example() {
        let mut lb = LetterBoxed::new(vec![vec!['o', 'l'], vec!['a', 'r'], vec!['p', 'd']]);
    }

    #[test]
    fn bigger_example() {
        let mut lb = LetterBoxed::new(vec![
            vec!['o', 'l', 't'],
            vec!['a', 'r', 'n'],
            vec!['p', 'd', 'c'],
            vec!['m', 'i', 'u'],
        ]);
    }
}
