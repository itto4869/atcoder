use std::collections::VecDeque;

use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut grid: [Bytes; h],
    }
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match grid[i][j] {
                b'S' => start = (i, j),
                b'G' => end = (i, j),
                b'^' => {
                    grid[i][j] = b'#';
                    for k in (0..i).rev() {
                        if grid[k][j] == b'.' {
                            grid[k][j] = b'x';
                        } else if grid[k][j] == b'x' {
                            continue;
                        } else {
                            break;
                        }
                    }
                },
                b'v' => {
                    grid[i][j] = b'#';
                    for k in (i + 1)..h {
                        if grid[k][j] == b'.' {
                            grid[k][j] = b'x';
                        } else if grid[k][j] == b'x' {
                            continue;
                        } else {
                            break;
                        }
                    }
                },
                b'<' => {
                    grid[i][j] = b'#';
                    for k in (0..j).rev() {
                        if grid[i][k] == b'.' {
                            grid[i][k] = b'x';
                        } else if grid[i][k] == b'x' {
                            continue;
                        } else {
                            break;
                        }
                    }
                },
                b'>' => {
                    grid[i][j] = b'#';
                    for k in (j + 1)..w {
                        if grid[i][k] == b'.' {
                            grid[i][k] = b'x';
                        } else if grid[i][k] == b'x' {
                            continue;
                        } else {
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut dist = vec![vec![u32::MAX; w]; h];
    dist[start.0][start.1] = 0;
    while let Some((r, c)) = queue.pop_front() {
        let next = neighbors4(r, c, h, w);
        for (nr, nc) in next {
            if grid[nr][nc] == b'#' || grid[nr][nc] == b'x' {
                continue;
            }

            if dist[nr][nc] <= dist[r][c] + 1 {
                continue;
            }

            dist[nr][nc] = dist[r][c] + 1;
            queue.push_back((nr, nc));
        }
    }

    if dist[end.0][end.1] == u32::MAX {
        println!("-1");
    } else {
        println!("{}", dist[end.0][end.1]);
    }
}
