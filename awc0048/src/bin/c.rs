use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [u64; n],
    }
    let mut imos = vec![0; n + 1];
    for _ in 0..m {
        input! {
            l: Usize1,
            r: Usize1,
            d: i64,
        }

        imos[l] += d;
        imos[r + 1] -= d;
    }

    for i in 1..=n {
        imos[i] += imos[i - 1];
    }

    let mut ans = 0;
    for i in 0..n {
        if h[i] > imos[i] as u64 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
