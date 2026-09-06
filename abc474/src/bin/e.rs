use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            ab: [(u64, u64); n],
        }

        // --------------------------
        // 追加購入が不要な場合
        // --------------------------
        let sum_b: u64 = ab.iter().map(|&(_, b)| b).sum();

        let mut d_all = ab
            .iter()
            .map(|&(a, b)| a - b)
            .collect::<Vec<_>>();

        d_all.sort_unstable();

        let s = (n + 1) / 2;

        let mut ans = sum_b + d_all[..s].iter().sum::<u64>();

        // N=1,2 では追加購入を考える必要がない
        if n >= 3 {
            // A_i が最小の商品
            let p = (0..n)
                .min_by_key(|&i| ab[i].0)
                .unwrap();

            let x = ab[p].0;

            // p 以外の d_i = A_i - B_i
            let mut d = Vec::with_capacity(n - 1);
            let mut sum_b_except_p = 0u64;

            for i in 0..n {
                if i == p {
                    continue;
                }

                let (a, b) = ab[i];
                d.push(a - b);
                sum_b_except_p += b;
            }

            d.sort_unstable();

            // pref[r] = 小さい方から r 個の d の和
            let mut pref = vec![0u64; n];

            for i in 0..(n - 1) {
                pref[i + 1] = pref[i] + d[i];
            }

            // --------------------------
            // 追加購入が必要な場合
            // --------------------------
            let r_max = (n - 3) / 2;

            for r in 0..=r_max {
                // 最小 A の商品を買う回数
                let k = n - 1 - 2 * r;

                let cost =
                    x * k as u64
                    + sum_b_except_p
                    + pref[r];

                ans = ans.min(cost);
            }
        }

        println!("{}", ans);
    }
}