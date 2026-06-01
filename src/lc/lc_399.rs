use std::collections::{HashMap, HashSet};

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut map = HashMap::with_capacity(40);
    equations
        .iter()
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .for_each(|&s| {
            map.insert(s, vec![]);
        });
    equations.iter().enumerate().for_each(|(i, v)| {
        let ab = map.get_mut(&v[0]).unwrap();
        ab.push((&v[1], values[i]));
        let ba = map.get_mut(&v[1]).unwrap();
        ba.push((&v[0], 1.0 / values[i]));
    });
    todo!()
}

#[cfg(test)]
mod test {
    use crate::lc::lc_399::calc_equation;

    #[test]
    fn test_calc_equation_1() {
        assert_eq!(
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")]
                ],
                vec![2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")]
                ]
            )
        );
    }
}
