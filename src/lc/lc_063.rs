pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut dg = vec![vec![0; n]; m];
    dg[0][0] = obstacle_grid[0][0] ^ 1;
    for i in 1..n {
        if obstacle_grid[0][i] == 1 {
            break;
        } else {
            dg[0][i] = dg[0][i - 1];
        }
    }
    for i in 1..m {
        if obstacle_grid[i][0] == 1 {
            break;
        } else {
            dg[i][0] = dg[i - 1][0];
        }
    }
    for i in 1..m {
        for ii in 1..n {
            if obstacle_grid[i][ii] == 1 {
                continue;
            }
            if obstacle_grid[i][ii - 1] == 0 || obstacle_grid[i - 1][ii] == 0 {
                dg[i][ii] = dg[i][ii - 1] + dg[i - 1][ii];
            }
        }
    }
    dg[m - 1][n - 1]
}

#[cfg(test)]
mod test {
    use crate::lc::lc_063::unique_paths_with_obstacles;

    #[test]
    fn test_unique_paths_with_obstacles_1() {
        assert_eq!(
            2,
            unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        );
    }

    #[test]
    fn test_unique_paths_with_obstacles_2() {
        assert_eq!(1, unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]));
    }
}
