use std::fs::read_to_string;

pub fn get_words() -> Vec<String> {
    read_to_string("words.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_line(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
        .to_lowercase()
}
