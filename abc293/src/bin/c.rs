use std::collections::HashSet;
use cp_library::grid::neighbors4;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[usize; w]; h],
    }

    let mut seen = vec![vec![false; w]; h];
    seen[0][0] = true;
    let mut set = HashSet::new();
    set.insert(grid[0][0]);
    let mut ans = 0;
    dfs(0, 0, h, w, &grid, &mut seen, &mut set, &mut ans);

    println!("{}", ans);
}

fn dfs(r: usize, c: usize, h: usize, w: usize, grid: &Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>, set: &mut HashSet<usize>, ans: &mut usize) {
    if (r, c) == (h - 1, w - 1) {
        *ans += 1;
        return;
    }
    
    let next = [(r, c + 1), (r + 1, c)];
    for (nr, nc) in next {
        if (nr >= h) || (nc >= w) {
            continue;
        }
        
        if seen[nr][nc] {
            continue;
        }

        let x = grid[nr][nc];
        if set.contains(&x) {
            continue;
        }

        set.insert(x);
        seen[nr][nc] = true;
        dfs(nr, nc, h, w, grid, seen, set, ans);
        set.remove(&x);
        seen[nr][nc] = false;
    }
}