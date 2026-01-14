use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut cnt = vec![0; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        cnt[(a + b) % n] += 1i64;
    }

    let mut ans = ((m * (m - 1)) / 2) as i64;
    for &e in &cnt {
        ans -= (e * (e - 1)) / 2;
    }

    println!("{}", ans);
}
