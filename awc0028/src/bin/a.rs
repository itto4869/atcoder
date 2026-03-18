use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        p: [u64; n],
    }
    let p = p.into_iter().filter(|&x| x >= k);
    let ans: u64 = p.sum();
    println!("{}", ans);
}
