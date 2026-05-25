pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![vec![0; 9]; 9];
    let mut cols = vec![vec![0; 9]; 9];
    let mut subs = vec![vec![vec![0; 9]; 3]; 3];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let val = board[i][j];
            if !val.eq(&'.') {
                let val = (val as usize) - 49;
                if rows[i][val] != 0 || cols[j][val] != 0 || subs[i / 3][j / 3][val] != 0 {
                    return false;
                } else {
                    rows[i][val] = 1;
                    cols[j][val] = 1;
                    subs[i / 3][j / 3][val] = 1;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::lc::lc_036::is_valid_sudoku;

    #[test]
    fn test_is_valid_sudoku_1() {
        assert_eq!(
            true,
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
    }
}
