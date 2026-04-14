use std::collections::VecDeque;

use cp_library::{grid::neighbors4, yes_no};
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let mut dist = vec![vec![-1i64; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    dist[0][0] = 0;
    let snuke = [b's', b'n', b'u', b'k', b'e'];
    while let Some((r, c)) = queue.pop_front() {
        if grid[r][c] != snuke[dist[r][c] as usize % 5] {
            break;
        }
        let next = neighbors4(r, c, h, w);
        for (nr, nc) in next {
            if dist[nr][nc] != -1 {
                continue;
            }

            if grid[nr][nc] != snuke[(dist[r][c] as usize + 1) % 5] {
                continue;
            }

            dist[nr][nc] = dist[r][c] + 1;
            queue.push_back((nr, nc));
        }
    }

    yes_no!(dist[h - 1][w - 1] != -1);
}
