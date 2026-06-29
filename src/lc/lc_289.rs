pub fn game_of_life(board: &mut [Vec<i32>]) {
    let mut update = vec![];
    let neighbors = [-1, 0, 1];
    let (m, n) = (board.len(), board[0].len());
    for i in 0..(m as i32) {
        for j in 0..(n as i32) {
            let mut lives = 0;
            for ii in 0..3 {
                for jj in 0..3 {
                    if !(neighbors[ii] == 0 && neighbors[jj] == 0) {
                        let r = i + neighbors[ii];
                        let c = j + neighbors[jj];
                        if r < 0 || c < 0 {
                            continue;
                        }
                        let r = r as usize;
                        let c = c as usize;
                        if r < m && c < n && board[r][c] == 1 {
                            lives += 1;
                        }
                    }
                }
            }

            let i = i as usize;
            let j = j as usize;
            if board[i][j] == 1 && !(2..=3).contains(&lives) {
                update.push((i, j));
            }
            if board[i][j] == 0 && lives == 3 {
                update.push((i, j));
            }
        }
    }
    update.iter().for_each(|&(i, j)| board[i][j] ^= 1);
}

#[cfg(test)]
mod test {
    use crate::lc::lc_289::game_of_life;

    #[test]
    fn test_game_of_life_1() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        game_of_life(&mut board);
        assert_eq!(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
            board
        );
    }
}
