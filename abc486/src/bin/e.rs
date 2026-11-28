use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            mut edges: [(Usize1, Usize1, u64); m],
        }

        edges.sort_unstable_by_key(|&(_, _, c)| c);

        let mut dp = vec![0u64; n];
        let mut next = vec![0u64; n];

        let mut l = 0;

        while l < m {
            let c = edges[l].2;
            let mut r = l;

            // 同じ重みの辺の範囲を求める
            while r < m && edges[r].2 == c {
                r += 1;
            }

            // 更新前の dp を参照して遷移を計算
            for &(u, v, c) in &edges[l..r] {
                next[u] = next[u].max(dp[v] + c);
                next[v] = next[v].max(dp[u] + c);
            }

            // 同じ重みの辺を一括で反映
            for &(u, v, _) in &edges[l..r] {
                dp[u] = dp[u].max(next[u]);
                dp[v] = dp[v].max(next[v]);
            }

            l = r;
        }

        println!("{}", dp.iter().max().unwrap());
    }
}