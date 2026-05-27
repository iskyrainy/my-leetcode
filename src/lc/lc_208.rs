use std::cell::RefCell;

struct Trie {
    ch: char,
    flag: bool,
    sons: Option<Vec<RefCell<Trie>>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            ch: ' ',
            flag: false,
            sons: None
        }
    }

    fn insert(&mut self, word: String) {
        for ch in word.chars() {
            
        }
        todo!()
    }

    fn search(&self, word: String) -> bool {
        todo!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_208::Trie;

    #[test]
    fn test_trie_1() {
        let trie = Trie::new();
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
