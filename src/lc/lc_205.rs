use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s2t = HashMap::new();
    let mut t2s = HashMap::new();
    for (sch, tch) in s.chars().zip(t.chars()) {
        match s2t.get(&sch) {
            Some(m_tch) => {
                if tch != *m_tch {
                    return false;
                }
            }
            None => {
                let _ = s2t.insert(sch, tch);
            }
        }
        match t2s.get(&tch) {
            Some(m_sch) => {
                if sch != *m_sch {
                    return false;
                }
            }
            None => {
                let _ = t2s.insert(tch, sch);
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::lc::lc_205::is_isomorphic;

    #[test]
    fn test_is_isomorphic_1() {
        assert!(is_isomorphic(String::from("egg"), String::from("pdd")));
        assert!(is_isomorphic(String::from("bad"), String::from("zyt")));
    }
}
