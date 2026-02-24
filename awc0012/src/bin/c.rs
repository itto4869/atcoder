use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
    }
    let mut h0 = Vec::with_capacity(n);
    let mut h1 = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            h: usize,
            p: u64,
        }
        if h == 0 {
            h0.push(p);
        } else {
            h1.push(p);
        }
    }

    h0.sort_unstable();
    h1.sort_unstable();

    h0.reverse();
    h1.reverse();

    if h0.len() < (k - m) || h1.len() < m {
        println!("-1");
    } else {
        let mut ans = 0;
        for i in 0..(k - m) {
            ans += h0[i];
        }

        for i in 0..m {
            ans += h1[i];
        }

        println!("{}", ans);
    }
}
