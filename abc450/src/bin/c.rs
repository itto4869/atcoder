use std::collections::HashSet;

use ac_library::Dsu;
use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut grid: [Bytes; h],
    }
    let mut dsu = Dsu::new(h * w);
    let idx = |r: usize, c: usize| {r * w + c};

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'#' {
                continue;
            }

            let next = neighbors4(i, j, h, w);
            for (nr, nc) in next {
                if grid[nr][nc] == b'.' {
                    dsu.merge(idx(i, j), idx(nr, nc));
                }
            }
        }
    }

    let mut set = HashSet::new();
    for j in 0..w {
        if grid[0][j] == b'.' {
            set.insert(dsu.leader(idx(0, j)));
        }

        if grid[h - 1][j] == b'.' {
            set.insert(dsu.leader(idx(h - 1, j)));
        }
    }

    for i in 0..h {
        if grid[i][0] == b'.' {
            set.insert(dsu.leader(idx(i, 0)));
        }

        if grid[i][w - 1] == b'.' {
            set.insert(dsu.leader(idx(i, w - 1)));
        }
    }

    let mut cnt_set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'.' {
                cnt_set.insert(dsu.leader(idx(i, j)));
            }
        }
    }

    let ans = cnt_set.len() - set.len();
    println!("{}", ans);
}
