use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let mut escapes = Vec::with_capacity(h * w);
    let mut ans = vec![vec![""; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'#' {
                ans[i][j] = "#";
            } else if grid[i][j] == b'E' {
                escapes.push((i, j));
                ans[i][j] = "E";
            }
        }
    }
    
    let dest = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let dest_c = HashMap::from([((-1, 0), "v"), ((0, 1), "<"), ((1, 0), "^"), ((0, -1), ">")]);
    let mut d_grid = vec![vec![u64::MAX; w]; h];

    let mut queue = VecDeque::new();
    for (e_y, e_x) in escapes {
        queue.push_back((e_y, e_x, 1));
    }
    while let Some((y, x, d)) = queue.pop_front() {
        for (dy, dx) in dest {
            if let (Some(ny), Some(nx)) = (y.checked_add_signed(dy), x.checked_add_signed(dx)) {
                if ny >= h || nx >= w {
                    continue;
                }

                if grid[ny][nx] == b'#' || grid[ny][nx] == b'E' {
                    continue;
                }

                if d_grid[ny][nx] > d {
                    d_grid[ny][nx] = d;
                    ans[ny][nx] = dest_c.get(&(dy, dx)).unwrap();
                    queue.push_back((ny, nx, d + 1));
                }

            }
        }
    }

    for row in ans {
        println!("{}", row.iter().format(""));
    }
}
