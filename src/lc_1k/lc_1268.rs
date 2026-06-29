pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let word_chs: Vec<char> = search_word.chars().collect();
    products.sort_unstable();
    let mut res = vec![vec![]; word_chs.len()];
    let mut tmp = String::new();
    for (i, &ch) in word_chs.iter().enumerate() {
        tmp.push(ch);
        for product in &products {
            if res[i].len() < 3 && product.starts_with(&tmp) {
                res[i].push(product.clone());
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::lc_1k::lc_1268::suggested_products;

    #[test]
    fn test_suggested_products_1() {
        assert_eq!(
            vec![
                vec![
                    String::from("mobile"),
                    String::from("moneypot"),
                    String::from("monitor")
                ],
                vec![
                    String::from("mobile"),
                    String::from("moneypot"),
                    String::from("monitor")
                ],
                vec![String::from("mouse"), String::from("mousepad")],
                vec![String::from("mouse"), String::from("mousepad")],
                vec![String::from("mouse"), String::from("mousepad")]
            ],
            suggested_products(
                vec![
                    String::from("mobile"),
                    String::from("mouse"),
                    String::from("moneypot"),
                    String::from("monitor"),
                    String::from("mousepad")
                ],
                String::from("mouse")
            )
        );
    }
}
