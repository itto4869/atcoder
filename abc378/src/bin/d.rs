use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        grid: [Chars; h],
    }
    let mut ans = 0;
    let mut visited = vec![vec![false; w]; h];
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == '.' {
                visited[r][c] = true;
                dfs(r, c, 0, h, w, k, &grid, &mut visited, &mut ans);
                visited[r][c] = false;
            }
        }
    }

    println!("{}", ans);
}

fn dfs(r: usize, c: usize, cnt: usize, h: usize, w: usize, k: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, ans: &mut usize) {
    if cnt == k {
        *ans = (*ans) + 1;
        return;
    }

    let next = neighbors4(r, c, h, w);
    for (nr, nc) in next {
        if visited[nr][nc] {
            continue;
        }

        if grid[nr][nc] == '#' {
            continue;
        }

        visited[nr][nc] = true;
        dfs(nr, nc, cnt + 1, h, w, k, grid, visited, ans);
        visited[nr][nc] = false;
    }
}