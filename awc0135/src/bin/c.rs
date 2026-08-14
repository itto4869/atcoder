use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [[usize; m]; n],
    }
    let mut queue = VecDeque::new();
    let mut v_grid = vec![vec![0; m]; n];
    queue.push_back((0, 0));
    v_grid[0][0] = grid[0][0];

    let dest = [(0, 1), (1, 0)];
    while let Some((r, c)) = queue.pop_front() {
        for &(dr, dc) in &dest {
            if (r + dr) >= n || (c + dc) >= m {
                continue;
            }

            let nr = r + dr;
            let nc = c + dc;

            if v_grid[nr][nc] >= (v_grid[r][c] + grid[nr][nc]) {
                continue;
            }

            v_grid[nr][nc] = v_grid[r][c] + grid[nr][nc];
            queue.push_back((nr, nc));
        }
    }

    println!("{}", v_grid[n - 1][m - 1]);
}
