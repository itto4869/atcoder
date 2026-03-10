use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1, Usize1); n],
    }
    let mut grid = vec![vec![false; w]; h];
    for (a, b) in ab {
        grid[a][b] = true;
    }

    let mut dp = vec![vec![0; w + 1]; h + 1];
    let mut ans = 0u64;
    for i in 1..=h {
        for j in 1..=w {
            if grid[i - 1][j - 1] {
                continue;
            }
            dp[i][j] = dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]) + 1;
            ans += dp[i][j];
        }
    }

    println!("{}", ans);
}
