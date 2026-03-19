use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
    }
    let mut ans = p;
    for _ in 0..n {
        input! {
            d: u64,
        }
        ans = ans.min(q + d);
    }

    println!("{}", ans);
}
