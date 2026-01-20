use std::collections::VecDeque;

use proconio::{fastout, input, marker::{Bytes, Usize1}};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
        a: Usize1,
        b :Usize1,
        c: Usize1,
        d: Usize1,
    }
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![-1; w]; h];
    let dest: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    dist[a][b] = 0;
    queue.push_back((a, b));
    while let Some((y, x)) = queue.pop_front() {
        if y == c && x == d {
            println!("{}", dist[y][x]);
            return;
        }

        for (dy, dx) in dest {
            if let (Some(ny), Some(nx)) = (y.checked_add_signed(dy), x.checked_add_signed(dx)) {
                if ny >= h || nx >= w {
                    continue;
                }

                if grid[ny][nx] == b'.' && (dist[ny][nx] == -1 || dist[ny][nx] > dist[y][x]) {
                    dist[ny][nx] = dist[y][x];
                    queue.push_front((ny, nx));
                } 

                if grid[ny][nx] == b'#' && (dist[ny][nx] == -1 || dist[ny][nx] > dist[y][x] + 1) {
                    dist[ny][nx] = dist[y][x] + 1;
                    queue.push_back((ny, nx));
                }

                if let (Some(nny), Some(nnx)) = (ny.checked_add_signed(dy), nx.checked_add_signed(dx)) {
                    if nny >= h || nnx >= w {
                        continue;
                    }
                    if dist[nny][nnx] == -1 || dist[nny][nnx] > dist[y][x] + 1 {
                        dist[nny][nnx] = dist[y][x] + 1;
                        queue.push_back((nny, nnx));
                    }
                }
            }
        }
    }
}
