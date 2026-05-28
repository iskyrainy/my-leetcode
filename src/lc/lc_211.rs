pub struct WordDictionary {
    pub flag: bool,
    pub sons: [Option<Box<WordDictionary>>; 26],
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            flag: false,
            sons: Default::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut now = self;
        for ch in word.as_bytes() {
            let i = (ch - b'a') as usize;
            if now.sons[i].is_none() {
                now.sons[i] = Some(Box::new(WordDictionary::new()));
            }
            now = now.sons[i].as_mut().unwrap();
        }
        now.flag = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::dfs(self, word.as_bytes(), 0)
    }

    fn dfs(wd: &Self, word: &[u8], index: usize) -> bool {
        if index == word.len() {
            return wd.flag;
        }
        let ch = word[index];
        match ch {
            b'.' => {
                for son in &wd.sons {
                    if let Some(son) = son {
                        if Self::dfs(&*son, word, index + 1) {
                            return true;
                        }
                    }
                }
            }
            _ => {
                let i = (ch - b'a') as usize;
                let son = &wd.sons[i];
                if let Some(son) = son {
                    if Self::dfs(&*son, word, index + 1) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::lc::lc_211::WordDictionary;

    #[test]
    fn test_word_dictionary_1() {
        let mut wd = WordDictionary::new();
        wd.add_word(String::from("app"));
        wd.add_word(String::from("apple"));
        wd.add_word(String::from("any"));
        assert_eq!(true, wd.search(String::from("apple")));
        assert_eq!(true, wd.search(String::from("app")));
        assert_eq!(true, wd.search(String::from("any")));
        assert_eq!(false, wd.search(String::from("bny")));
        assert_eq!(true, wd.search(String::from(".pp.e")));
    }
}
