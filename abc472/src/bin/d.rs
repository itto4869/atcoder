use std::collections::VecDeque;

use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        grid: [Chars; h],
    }
    let mut v_h = vec![true; w];
    let mut v_w = vec![true; h];

    for i in 0..h {
        let mut ok = true;
        for j in 0..w {
            if grid[i][j] == '#' {
                ok = false;
                break;
            }
        }
        v_w[i] = ok;
    }

    for j in 0..w {
        let mut ok = true;
        for i in 0..h {
            if grid[i][j] == '#' {
                ok = false;
                break;
            }
        }
        v_h[j] = ok;
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![vec![1usize << 60; w]; h];
    for i in 0..h {
        for j in 0..w {
            if v_h[j] & v_w[i] {
                queue.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        let next = neighbors4(r, c, h, w);
        for (nr, nc) in next {
            if grid[nr][nc] == '#' {
                continue;
            }

            if dist[nr][nc] <= (dist[r][c] + 1) {
                continue;
            }

            dist[nr][nc] = dist[r][c] + 1;
            queue.push_back((nr, nc));
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
