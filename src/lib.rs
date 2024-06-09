mod letterbox;
mod trie;
mod words;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::letterbox::LetterBoxed;

    #[test]
    fn smaller_example() {
        let mut lb = LetterBoxed::new(vec![vec!['t', 'o', 'i'], vec!['a', 'l', 'g']]);
        println!("{:?}", lb.solve());
    }

    #[test]
    fn bigger_example() {
        let mut lb = LetterBoxed::new(vec![
            vec!['o', 'q', 'e'],
            vec!['m', 'i', 'a'],
            vec!['l', 'u', 'r'],
            vec!['z', 'c', 'h'],
        ]);
        println!("{:?}", lb.solve());
    }
}
