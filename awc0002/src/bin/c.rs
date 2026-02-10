use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        ab: [(u64, u64); n],
    }
    let mut ans = 0;
    for &(a, b) in &ab {
        let d = (m.saturating_sub(a) + b - 1) / b;
        ans = ans.max(d);
    }

    println!("{}", ans);
}
