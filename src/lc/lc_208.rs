#[derive(Debug, Clone)]
pub struct Trie {
    pub flag: bool,
    pub sons: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            flag: false,
            sons: Default::default(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut now = self;
        for ch in word.as_bytes() {
            let i = (ch - b'a') as usize;
            if now.sons[i].is_none() {
                now.sons[i] = Some(Box::new(Trie::new()));
            }
            now = now.sons[i].as_mut().unwrap();
        }
        now.flag = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut now = self;
        for ch in word.as_bytes() {
            let i = (ch - b'a') as usize;
            match &now.sons[i] {
                Some(node) => now = node,
                None => return false,
            }
        }
        now.flag
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut now = self;
        for ch in prefix.as_bytes() {
            let i = (ch - b'a') as usize;
            match &now.sons[i] {
                Some(node) => now = node,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_208::Trie;

    #[test]
    fn test_trie_1() {
        let mut trie = Trie::new();
        let apple = String::from("apple");
        let app = String::from("app");
        trie.insert(apple.clone());
        assert_eq!(true, trie.search(apple));
        assert_eq!(false, trie.search(app.clone()));
        assert_eq!(true, trie.starts_with(app.clone()));
        trie.insert(app.clone());
        assert_eq!(true, trie.search(app));
    }
}
