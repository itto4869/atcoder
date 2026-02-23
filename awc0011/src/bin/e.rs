use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, u64); n],
    }

    // 左からのDP
    let mut dp_l = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        let (w, v) = ab[i];
        for j in 0..=m {
            dp_l[i + 1][j] = dp_l[i][j]; // 選ばない場合
            if j >= w {
                dp_l[i + 1][j] = dp_l[i + 1][j].max(dp_l[i][j - w] + v); // 選ぶ場合
            }
        }
    }

    // 右からのDP
    let mut dp_r = vec![vec![0; m + 1]; n + 1];
    for i in (0..n).rev() {
        let (w, v) = ab[i];
        for j in 0..=m {
            dp_r[i][j] = dp_r[i + 1][j]; // 選ばない場合
            if j >= w {
                dp_r[i][j] = dp_r[i][j].max(dp_r[i + 1][j - w] + v); // 選ぶ場合
            }
        }
    }

    let max_v = dp_l[n][m];

    // 各宝箱についての判定
    for i in 0..n {
        let (w, v) = ab[i];
        let mut is_optimal = false;
        
        for j in 0..=(m - w) { // jは宝箱iより左側で使う重さ
            // 左側の最大価値 + 右側の最大価値 + 宝箱iの価値 == 全体の最大価値 になるか？
            if dp_l[i][j] + dp_r[i + 1][m - w - j] + v == max_v {
                is_optimal = true;
                break;
            }
        }

        if is_optimal {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}