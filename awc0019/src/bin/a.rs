use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        s: [u64; n],
    }
    let ans = s.into_iter().filter(|&x| x >= k).count();
    println!("{}", ans);
}
