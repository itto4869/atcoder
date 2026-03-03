use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // ルーレットの情報を格納
    let mut roulettes = vec![];
    for _ in 0..n {
        input! {
            c: f64,
            p: usize,
            s: [usize; p],
        }
        roulettes.push((c, p, s));
    }

    // dp[x] : 残り x ポイント必要なときの期待値の最小値
    let mut dp = vec![0.0; m + 1];

    for x in 1..=m {
        dp[x] = std::f64::INFINITY;
        
        for (c, p, s) in &roulettes {
            let mut expected_sum = 0.0;
            let mut zero_count = 0;

            for &points in s {
                if points == 0 {
                    zero_count += 1;
                } else {
                    // x より大きなポイントが出た場合は残り0ポイントとして扱う
                    let next_x = x.saturating_sub(points);
                    expected_sum += dp[next_x];
                }
            }

            // 式に基づいて E_i[x] を計算
            let p_f64 = *p as f64;
            let e_i = (p_f64 * c + expected_sum) / (p_f64 - zero_count as f64);
            
            if e_i < dp[x] {
                dp[x] = e_i;
            }
        }
    }

    println!("{:.10}", dp[m]);
}