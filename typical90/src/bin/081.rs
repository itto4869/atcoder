use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut imos = vec![vec![0u64; 5001]; 5001];
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        imos[a][b] += 1;
    }

    for i in 0..=5000 {
        for j in 1..=5000 {
            imos[i][j] += imos[i][j - 1];
        }
    }

    for i in 1..=5000 {
        for j in 0..=5000 {
            imos[i][j] += imos[i - 1][j];
        }
    }

    let mut ans = 0;
    for i in 1..=5000 {
        for j in 1..=5000 {
            let cnt = imos[i][j] + imos[i.saturating_sub(k + 1)][j.saturating_sub(k + 1)] - imos[i.saturating_sub(k + 1)][j] - imos[i][j.saturating_sub(k + 1)];
            ans = ans.max(cnt);
        }
    }
    println!("{}", ans);
}
