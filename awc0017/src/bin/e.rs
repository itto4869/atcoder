use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let cost = (xy[i].0 - xy[j].0) * (xy[i].0 - xy[j].0) + (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
            dist[i][j] = cost;
        }
    }

    let mut dp = vec![vec![1 << 60; n]; 1 << n];
    dp[1][0] = 0;

    for s in 1..(1 << n) {
        for u in 0..n {
            for v in 0..n {
                if (s >> u & 1 != 0) && (s >> v & 1 == 0) {
                    let ns = s | 1 << v;
                    dp[ns][v] = dp[ns][v].min(dp[s][u] + dist[u][v]);
                }
            }
        }
    }

    let mut ans = i64::MAX;
    for v in 0..n {
        ans = ans.min(dp[(1 << n) - 1][v] + dist[v][0]);
    }

    println!("{}", ans);
}
