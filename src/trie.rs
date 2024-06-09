#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

#[derive(Default, Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_key: bool,
}

pub fn get_num_let(c: usize) -> u8 {
    (c as u8) + b'a'
}

pub fn get_let_num(c: u8) -> usize {
    (c as usize) - ('a' as usize)
}

impl TrieNode {
    fn insert(&mut self, key: &str, pos: u8) {
        if pos as usize == key.len() {
            self.is_key = true;
        } else {
            let cur_ind = get_let_num(key.as_bytes()[pos as usize]);
            if self.children[cur_ind].is_none() {
                self.children[cur_ind] = Some(Box::default())
            }
            self.children[cur_ind]
                .as_mut()
                .unwrap()
                .insert(key, pos + 1)
        }
    }

    fn query(&self, key: &str, pos: u8) -> Option<&TrieNode> {
        if pos as usize == key.len() {
            Some(self)
        } else {
            let child = &self.children[get_let_num(key.as_bytes()[pos as usize])];
            if child.is_some() {
                child.as_ref().unwrap().query(key, pos + 1)
            } else {
                None
            }
        }
    }

    fn get_children(&self, key: &str) -> Option<Vec<(usize, &Box<TrieNode>)>> {
        Some(
            self.query(key, 0)?
                .children
                .iter()
                .enumerate()
                .filter(|(_, x)| x.is_some())
                .map(|(i, x)| (i, x.as_ref().unwrap()))
                .collect(),
        )
    }

    /// @return a Vec of all reversed keys
    /// keys reversed for perf
    fn list_reversed_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        if self.is_key {
            keys.push(String::new());
        }
        for (i, child) in self.get_children("").unwrap() {
            for mut child_key in child.list_reversed_keys() {
                child_key.push(get_num_let(i) as char);
                keys.push(child_key);
            }
        }
        keys
    }
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn from_vec(vec: Vec<String>) -> Trie {
        let mut trie = Trie::new();
        for s in vec {
            trie.insert(&s);
        }
        trie
    }

    pub fn is_node(&self, key: &str) -> bool {
        self.root.query(key, 0).is_some()
    }

    pub fn is_key(&self, key: &str) -> bool {
        let node = self.root.query(key, 0);
        node.is_some() && node.unwrap().is_key
    }

    pub fn insert(&mut self, key: &str) {
        self.root.insert(key, 0)
    }

    pub fn get_children(&self, key: &str) -> Option<Vec<u8>> {
        Some(
            self.root
                .get_children(key)?
                .into_iter()
                .enumerate()
                .map(|(i, _)| get_num_let(i))
                .collect(),
        )
    }

    pub fn list_reversed_keys(&self, key: &str) -> Option<Vec<String>> {
        Some(self.root.query(key, 0)?.list_reversed_keys())
    }
}
