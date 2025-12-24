use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[u64; w]; h],
    }

    let mut total_xor = 0;
    for i in 0..h {
        for j in 0..w {
            total_xor ^= grid[i][j];
        }
    }

    let mut used = vec![vec![false; w]; h];

    let mut ans = 0;

    dfs(0, total_xor, h, w, &mut ans, &mut used, &grid);

    println!("{}", ans);

}

fn dfs(idx: usize, curr: u64, h: usize, w: usize, ans: &mut u64, used: &mut Vec<Vec<bool>>, grid: &Vec<Vec<u64>>) {
    if idx == h * w {
        if *ans < curr {
            *ans = curr;
        }
        return
    }

    let r = idx / w;
    let c = idx % w;

    if used[r][c] {
        dfs(idx + 1, curr, h, w, ans, used, grid);
        return;
    }

    dfs(idx + 1, curr, h, w, ans, used, grid);

    if c + 1 < w && !used[r][c + 1] {
        used[r][c] = true;
        used[r][c + 1] = true;
        let new_val = curr ^ grid[r][c] ^ grid[r][c + 1];

        dfs(idx + 1, new_val, h, w, ans, used, grid);

        used[r][c] = false;
        used[r][c + 1] = false;
    }

    if r + 1 < h && !used[r + 1][c] {
        used[r][c] = true;
        used[r + 1][c] = true;

        let new_val = curr ^ grid[r][c] ^ grid[r + 1][c];

        dfs(idx + 1, new_val, h, w, ans, used, grid);

        used[r][c] = false;
        used[r + 1][c] = false;
    }
}