use std::collections::VecDeque;

use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![0; w]; h];
    queue.push_back((0, 0));
    dist[0][0] = 1;
    while let Some((r, c)) = queue.pop_front() {
        let next = neighbors4(r, c, h, w);
        for (nr, nc) in next {
            if dist[nr][nc] != 0 {
                continue;
            }

            if grid[nr][nc] == '#' {
                continue;
            }

            dist[nr][nc] = dist[r][c] + 1;
            queue.push_back((nr, nc));
        }
    }

    let d = dist[h - 1][w - 1];
    let mut cnt = 0;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == '.' {
                cnt += 1;
            }
        }
    }

    let ans = cnt - d;
    if d == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
