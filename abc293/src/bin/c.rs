use std::collections::HashSet;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[usize; w]; h],
    }

    let mut visited = vec![vec![false; w]; h];
    visited[0][0] = true;
    let mut ans = 0;
    let mut set = HashSet::new();
    set.insert(grid[0][0]);
    dfs(0, 0, h, w, &mut visited, &mut ans, &mut set, &grid);
    println!("{}", ans);
}

fn dfs(r: usize, c: usize, h: usize, w: usize, visited: &mut Vec<Vec<bool>>, ans: &mut usize, set: &mut HashSet<usize>, grid: &Vec<Vec<usize>>) {
    if (r, c) == (h - 1, w - 1) {
        *ans = (*ans) + 1;
        return;
    }
    let next = [(r, c + 1), (r + 1, c)];
    for (nr, nc) in next {
        if nr >= h || nc >= w {
            continue;
        }
        if visited[nr][nc] {
            continue;
        }

        let k = grid[nr][nc];
        if set.contains(&k) {
            continue;
        }

        set.insert(k);
        visited[nr][nc] = true;
        dfs(nr, nc, h, w, visited, ans, set, grid);
        set.remove(&k);
        visited[nr][nc] = false;
    }
}