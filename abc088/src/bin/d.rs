use std::collections::VecDeque;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let next: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dist: Vec<Vec<isize>> = vec![vec![-1; w]; h];
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back((0, 0));
    dist[0][0] = 1;
    while let Some((ny, nx)) = queue.pop_front() {
        for (dy, dx) in next {
            if ny + dy < 0 || ny + dy >= h as isize || nx + dx < 0 || nx + dx >= w as isize {
                continue;
            } else {
                let nny = ny + dy;
                let nnx = nx + dx;
                if grid[nny as usize][nnx as usize] == b'#' {
                    continue;
                } else if dist[nny as usize][nnx as usize] != -1 {
                    continue;
                } else {
                    dist[nny as usize][nnx as usize] = dist[ny as usize][nx as usize] + 1;
                    queue.push_back((nny, nnx));
                }
            }
        }
    }

    let mut black_num = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'#' {
                black_num += 1;
            }
        }
    }

    if dist[h - 1][w - 1] == -1 {
        println!("{}", -1);
    } else {
        println!("{}", (h * w) as isize - dist[h - 1][w - 1] - black_num);
    }
}
