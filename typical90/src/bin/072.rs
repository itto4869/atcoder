use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }

    let mut ans = -1;
    for sy in 0..h {
        for sx in 0..w {
            if grid[sy][sx] == b'#' {
                continue;
            }

            let mut seen = vec![vec![false; w]; h];
            seen[sy][sx] = true;
            dfs(sx, sy, sx, sy, 1, &grid, &mut seen, &mut ans);
        }
    }

    println!("{}", ans);
}

fn dfs(sx: usize, sy: usize, x: usize, y: usize, len: i64, grid: &Vec<Vec<u8>>, seen: &mut Vec<Vec<bool>>, ans: &mut i64) {
    let h = grid.len();
    let w = grid[0].len();

    for (ny, nx) in neighbors4(y, x, h, w) {
        if grid[ny][nx] == b'#' {
            continue;
        }

        if nx == sx && ny == sy {
            if len >= 3 {
                *ans = (*ans).max(len);
            }
            continue;
        }

        if !seen[ny][nx] {
            seen[ny][nx] = true;
            dfs(sx, sy, nx, ny, len + 1, grid, seen, ans);
            seen[ny][nx] = false;
        }
    }
}