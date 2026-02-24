use std::collections::VecDeque;

use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Bytes; n],
    }
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    let mut dist = vec![vec![-1; m]; n];
    dist[0][0] = 0;
    while let Some((r, c)) = queue.pop_front() {
        let next = neighbors4(r, c, n, m);
        for (nr, nc) in next {
            if dist[nr][nc] != -1 {
                continue;
            }

            if grid[nr][nc] == b'#' {
                dist[nr][nc] = dist[r][c] + 1;
                queue.push_back((nr, nc));
            } else {
                dist[nr][nc] = dist[r][c];
                queue.push_front((nr, nc));
            }
        }
    }

    println!("{}", dist[n - 1][m - 1]);
}
