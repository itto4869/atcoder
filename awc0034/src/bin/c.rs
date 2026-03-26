use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: u64,
        c: u64,
        a: [u64; n],
    }
    let mut imos = vec![0i64; n + 1];
    let mut ans = 0;
    for i in 0..n {
        if i > 0 {
            imos[i] += imos[i - 1];
        }
        let d = t.saturating_sub(a[i] + imos[i] as u64);

        imos[i] += d as i64;
        imos[(i + k).min(n)] -= d as i64;

        ans += d * c;
    }

    println!("{}", ans);
}
