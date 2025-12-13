use std::collections::{HashMap, VecDeque};

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let mut warp = vec![Vec::new(); 26];
    let mut map = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != b'.' && grid[i][j] != b'#' {
                let idx = alpha_to_idx(grid[i][j]);
                warp[idx].push((i, j));
            }
        }
    }

    for i in 0..26 {
        map.insert(i, warp[i].clone());
    }

    let mut dist = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();
    dist[0][0] = 0;
    queue.push_back((0, 0));

    let dest: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in dest {
            let ny = y + dy;
            let nx = x + dx;
            if ny < 0 || ny >= h as isize || nx < 0 || nx >= w as isize {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;

            if grid[ny][nx] == b'#' {
                continue;
            }

            if dist[ny][nx] != -1 {
                continue;
            }

            dist[ny][nx] = dist[y as usize][x as usize] + 1;
            queue.push_back((ny as isize, nx as isize));
        }

        let y = y as usize;
        let x = x as usize;
        if grid[y][x] != b'.' {
            let c = grid[y][x];
            let idx = alpha_to_idx(c);
            let mut warp = Vec::new();
            for &(ny, nx) in map.get(&idx).unwrap() {
                if (ny, nx) == (y as usize, x) {
                    warp.push((ny, nx));
                    continue;
                }

                if dist[ny][nx] != -1 {
                    continue;
                }

                warp.push((ny, nx));
                dist[ny][nx] = dist[y][x] + 1;
                queue.push_back((ny as isize, nx as isize));
            }
            map.insert(idx, warp);
        }
    }

    println!("{}", dist[h - 1][w - 1]);
}

fn alpha_to_idx(c: u8) -> usize {
    return (c - b'a') as usize
}